use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::rc::Rc;

use foxhole_production_calculator_types::Material::{self, *};
use foxhole_production_calculator_types::{
    BuildCost, Input, Output, ProductionChannel, Structure, Upgrade,
};
use indextree::{Arena, Node, NodeId};
use itertools::sorted;
use serde::Serialize;

include!(concat!(env!("OUT_DIR"), "/structures.rs"));

#[derive(Debug, Clone)]
pub struct StructureKey {
    parent: Option<String>,
    upgrade: String,
    prod_channel_idx: usize,
    output: Output,
}

impl StructureKey {
    fn structure_name(&self) -> String {
        match &self.parent {
            Some(parent) => format!("{}-{}", parent, self.upgrade),
            None => self.upgrade.clone(),
        }
    }
}

impl PartialEq for StructureKey {
    fn eq(&self, other: &Self) -> bool {
        self.parent == other.parent
            && self.upgrade == other.upgrade
            && self.prod_channel_idx == other.prod_channel_idx
    }
}

impl Eq for StructureKey {}

impl Hash for StructureKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.parent.hash(state);
        self.upgrade.hash(state);
        self.prod_channel_idx.hash(state);
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct StructureTree {
    pub arena: Arena<StructureTreeNode>,
    pub roots: Option<Vec<NodeId>>,
}

impl StructureTree {
    pub fn activate_node(&mut self, node_id: NodeId) {
        let upgrade_options = {
            let node = self.arena.get(node_id).expect("Node should exist").get();
            // Rc clone is a little sloppy here, but avoids carrying a reference on self
            node.upgrade_options.clone()
        };
        let upgrade_options = upgrade_options.borrow_mut();
        for option_node_id in upgrade_options.iter() {
            let node = self
                .arena
                .get_mut(*option_node_id)
                .expect("Node should exist")
                .get_mut();

            if *option_node_id == node_id {
                node.active = true;
            } else {
                node.active = false;
            }
        }
    }

    pub fn get_node(&self, node_id: NodeId) -> Option<&StructureTreeNode> {
        self.arena.get(node_id).map(|node| node.get())
    }

    pub fn get_arena_node(&self, node_id: NodeId) -> Option<&Node<StructureTreeNode>> {
        self.arena.get(node_id)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct StructureTreeNode {
    structure: StructureKey,
    count: f32,
    active: bool,
    upgrade_options: Rc<RefCell<Vec<NodeId>>>,
}

impl StructureTreeNode {
    pub fn structure_name(&self) -> String {
        self.structure.structure_name()
    }

    pub fn count(&self) -> f32 {
        self.count
    }

    pub fn is_active(&self) -> bool {
        self.active
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct FactoryRequirementsBuilding {
    pub building: String,
    pub upgrade: Option<String>,
    pub count: f32,
}

impl PartialEq for FactoryRequirementsBuilding {
    fn eq(&self, other: &Self) -> bool {
        self.building == other.building
            && self.upgrade == other.upgrade
            && self.count == other.count
    }
}

impl Eq for FactoryRequirementsBuilding {}

impl PartialOrd for FactoryRequirementsBuilding {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for FactoryRequirementsBuilding {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.building
            .cmp(&other.building)
            .then(self.upgrade.cmp(&other.upgrade))
    }
}

#[derive(Debug, Serialize, PartialEq)]
pub struct FactoryRequirements {
    pub buildings: Vec<FactoryRequirementsBuilding>,
    pub power: f32,
    pub build_cost: HashMap<Material, u64>,
    pub inputs: HashMap<Material, f32>,
}

pub struct ResourceGraph<'a> {
    structure_map: &'a HashMap<String, &'a Structure>,
    upgrade_map: &'a HashMap<Material, Vec<Upgrade>>,
}

impl<'a> Default for ResourceGraph<'a> {
    fn default() -> Self {
        Self {
            structure_map: &*STRUCTURE_MAP,
            upgrade_map: &*OUTPUT_MAP,
        }
    }
}

impl<'a> ResourceGraph<'a> {
    #[cfg(test)]
    fn new(
        structure_map: &'a HashMap<String, &'a Structure>,
        upgrade_map: &'a HashMap<Material, Vec<Upgrade>>,
    ) -> Self {
        Self {
            structure_map,
            upgrade_map,
        }
    }

    /// Calculate factory requirements given a material and a rate.
    ///
    /// Rate is assumed to be unit/hour.
    pub fn calculate_factory_requirements(
        &self,
        outputs: HashMap<Material, u64>,
        user_inputs: HashSet<Material>,
    ) -> Vec<StructureTree> {
        let mut trees = Vec::new();

        for (output, rate) in outputs.into_iter() {
            let mut tree = StructureTree::default();
            let mut stack = vec![(output, rate as f32, None)];
            self.traverse_building_reqs(&mut stack, &user_inputs, &mut tree);
            trees.push(tree);
        }

        trees
    }

    pub fn factory_requirements_from_trees(
        &self,
        trees: &[StructureTree],
        user_inputs: HashSet<Material>,
    ) -> FactoryRequirements {
        let mut build_costs = HashMap::new();
        let mut power = 0.0;
        let mut inputs = HashMap::new();
        for tree in trees {
            let roots = if let Some(roots) = &tree.roots {
                roots
            } else {
                break;
            };
            for root in roots {
                let mut stack = vec![*root];
                while let Some(node_id) = stack.pop() {
                    let node = tree.arena.get(node_id).expect("Node should exist").get();
                    if !node.active {
                        continue;
                    }
                    for child in node_id.children(&tree.arena) {
                        stack.push(child);
                    }

                    if let Some(parent) = &node.structure.parent {
                        // Non-default upgrade case
                        let structure = self
                            .structure_map
                            .get(parent)
                            .expect("Structure should exist");
                        let upgrade = structure
                            .upgrades
                            .get(&node.structure.upgrade)
                            .expect("Upgrade should exist");

                        calculate_build_costs(
                            &mut build_costs,
                            &structure.default_upgrade,
                            node.count,
                        );
                        calculate_build_costs(&mut build_costs, upgrade, node.count);

                        power += upgrade.production_channels[node.structure.prod_channel_idx].power
                            * node.count.ceil();
                    } else {
                        // Default upgrade case
                        let structure = self
                            .structure_map
                            .get(&node.structure.upgrade)
                            .expect("Structure should exist");

                        calculate_build_costs(
                            &mut build_costs,
                            &structure.default_upgrade,
                            node.count,
                        );

                        let production_channel = &structure.default_upgrade.production_channels
                            [node.structure.prod_channel_idx];
                        power += production_channel.power * node.count.ceil();
                    }

                    // Calculate inputs
                    let upgrade = if let Some(parent) = &node.structure.parent {
                        let structure = self
                            .structure_map
                            .get(parent)
                            .expect("Structure should exist");

                        structure
                            .upgrades
                            .get(&node.structure.upgrade)
                            .expect("Upgrade should exist")
                    } else {
                        let structure = self
                            .structure_map
                            .get(&node.structure.upgrade)
                            .expect("Structure should exist");

                        &structure.default_upgrade
                    };

                    let production_channel =
                        &upgrade.production_channels[node.structure.prod_channel_idx];

                    for input in &production_channel.inputs {
                        if !self.upgrade_map.contains_key(&input.material)
                            || user_inputs.contains(&input.material)
                        {
                            let rate = production_channel.hourly_rate(input.value) * node.count;
                            let entry = inputs.entry(input.material).or_default();

                            *entry += rate;
                        }
                    }
                }
            }
        }

        // Dedupe structures
        let mut building_map = HashMap::new();
        for tree in trees {
            let roots = if let Some(roots) = &tree.roots {
                roots
            } else {
                break;
            };
            for root in roots {
                for edge in root.traverse(&tree.arena) {
                    match edge {
                        indextree::NodeEdge::Start(node_id) => {
                            let node = tree.arena.get(node_id).expect("Node should exist").get();
                            if !node.active {
                                continue;
                            }
                            if let Some(parent) = node.structure.parent.clone() {
                                let entry: &mut f32 = building_map
                                    .entry((parent, Some(node.structure.upgrade.clone())))
                                    .or_default();

                                *entry += node.count;
                            } else {
                                let entry = building_map
                                    .entry((node.structure.upgrade.clone(), None))
                                    .or_default();

                                *entry += node.count;
                            }
                        }
                        indextree::NodeEdge::End(_node_id) => {}
                    }
                }
            }
        }

        //Sort here to avoid non-determinism in test and outputs.
        let buildings: Vec<FactoryRequirementsBuilding> = sorted(building_map.into_iter().map(
            |((structure, upgrade), count)| FactoryRequirementsBuilding {
                building: structure,
                upgrade,
                count,
            },
        ))
        .collect();

        FactoryRequirements {
            buildings,
            power,
            build_cost: build_costs,
            inputs,
        }
    }

    fn traverse_building_reqs(
        &self,
        stack: &mut Vec<(Material, f32, Option<NodeId>)>,
        user_inputs: &HashSet<Material>,
        tree: &mut StructureTree,
    ) {
        while let Some((current_input, current_rate, parent_node)) = stack.pop() {
            if let Some(upgrades) = self.upgrade_map.get(&current_input) {
                if !user_inputs.contains(&current_input) {
                    self.calculate_building_counts(
                        upgrades,
                        current_input,
                        current_rate,
                        stack,
                        tree,
                        parent_node,
                    );
                }
            }
        }
    }

    fn calculate_building_counts(
        &self,
        upgrades: &[Upgrade],
        current_input: Material,
        current_rate: f32,
        stack: &mut Vec<(Material, f32, Option<NodeId>)>,
        tree: &mut StructureTree,
        parent_node: Option<NodeId>,
    ) {
        let mut upgrade_list = Vec::new();
        let upgrade_options = Rc::new(RefCell::new(Vec::new()));
        for upgrade in upgrades {
            for (prod_channel_idx, production_channel) in
                upgrade.production_channels.iter().enumerate()
            {
                // FIXME: This sucks, change outputs to be a map
                for output in &production_channel.outputs {
                    if current_input == output.material {
                        let structure_key = StructureKey {
                            parent: upgrade.parent.clone(),
                            upgrade: upgrade.name.clone(),
                            prod_channel_idx,
                            output: output.clone(),
                        };

                        let output_val = production_channel.hourly_rate(output.value);
                        upgrade_list.push((output_val, structure_key));
                    }
                }
            }
        }

        upgrade_list.sort_by(|(a, _), (b, __)| a.partial_cmp(b).unwrap());
        let mut upgrade_iter = upgrade_list.into_iter().rev();
        let structure_key = upgrade_iter.next().unwrap().1;
        self.push_upgrade_to_tree(
            current_rate,
            tree,
            stack,
            &parent_node,
            structure_key,
            upgrade_options.clone(),
            true,
        );

        for (_, structure_key) in upgrade_iter {
            self.push_upgrade_to_tree(
                current_rate,
                tree,
                stack,
                &parent_node,
                structure_key,
                upgrade_options.clone(),
                false,
            );
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn push_upgrade_to_tree(
        &self,
        current_rate: f32,
        tree: &mut StructureTree,
        stack: &mut Vec<(Material, f32, Option<NodeId>)>,
        parent_node: &Option<NodeId>,
        structure_key: StructureKey,
        upgrade_options: Rc<RefCell<Vec<NodeId>>>,
        active: bool,
    ) {
        let production_channel = if let Some(parent) = &structure_key.parent {
            let structure = self
                .structure_map
                .get(parent)
                .expect("Structure should exist");
            let upgrade = structure
                .upgrades
                .get(&structure_key.upgrade)
                .expect("Upgrade should exist");

            upgrade.production_channels[structure_key.prod_channel_idx].clone()
        } else {
            let structure = self
                .structure_map
                .get(&structure_key.upgrade)
                .expect("Structure should exist");

            structure.default_upgrade.production_channels[structure_key.prod_channel_idx].clone()
        };

        let output_value = structure_key.output.value;
        let building_count = current_rate as f32 / production_channel.hourly_rate(output_value);
        let node = StructureTreeNode {
            structure: structure_key,
            count: building_count,
            active,
            upgrade_options,
        };
        let node_id = tree.arena.new_node(node);
        let node = tree
            .arena
            .get_mut(node_id)
            .expect("Node should exist")
            .get_mut();
        node.upgrade_options.borrow_mut().push(node_id);
        if let Some(parent_node_id) = parent_node {
            parent_node_id.append(node_id, &mut tree.arena);
        } else {
            let roots = tree.roots.get_or_insert_with(Vec::new);
            roots.push(node_id);
        }

        for input in &production_channel.inputs {
            stack.push((
                input.material,
                production_channel.hourly_rate(input.value) * building_count,
                Some(node_id),
            ));
        }
    }
}

fn calculate_build_costs(
    build_costs: &mut HashMap<Material, u64>,
    upgrade: &Upgrade,
    upgrade_count: f32,
) {
    for build_cost in &upgrade.build_costs {
        let entry = build_costs.entry(build_cost.material).or_default();

        *entry += build_cost.cost * upgrade_count.ceil() as u64;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use foxhole_production_calculator_types::BuildCost;

    fn build_structures() -> Vec<Structure> {
        let upgrade_a = Upgrade::new(
            "upgrade_a".to_string(),
            vec![BuildCost::new(Material::BasicMaterials, 1)],
            vec![ProductionChannel {
                power: 1.0,
                rate: 3600,
                inputs: vec![Input::new(Material::Coal, 1)],
                outputs: vec![Output::new(Material::Coke, 1)],
            }],
            None,
        );

        let upgrade_a_1 = Upgrade::new(
            "upgrade_a_1".to_string(),
            vec![BuildCost::new(Material::BasicMaterials, 1)],
            vec![ProductionChannel {
                power: 1.0,
                rate: 3600,
                inputs: vec![Input::new(Material::Coal, 1)],
                outputs: vec![Output::new(Material::Coke, 2)],
            }],
            Some("upgrade_a".to_string()),
        );

        let upgrade_b = Upgrade::new(
            "upgrade_b".to_string(),
            vec![BuildCost::new(Material::BasicMaterials, 1)],
            vec![
                ProductionChannel {
                    power: 1.0,
                    rate: 3600,
                    inputs: vec![Input::new(Material::Components, 1)],
                    outputs: vec![Output::new(Material::Rocket4CFire, 1)],
                },
                ProductionChannel {
                    power: 1.0,
                    rate: 3600,
                    inputs: vec![Input::new(Material::Components, 1)],
                    outputs: vec![Output::new(Material::Rocket3CHighExplosive, 1)],
                },
            ],
            None,
        );

        let upgrade_c = Upgrade::new(
            "upgrade_c".to_string(),
            vec![BuildCost::new(Material::BasicMaterials, 1)],
            vec![ProductionChannel {
                power: 1.0,
                rate: 3600,
                inputs: vec![Input::new(Material::Coke, 1)],
                outputs: vec![Output::new(Material::ConcreteMaterials, 1)],
            }],
            None,
        );

        let structure_a = Structure::new(
            upgrade_a,
            vec![("upgrade_a_1".to_string(), upgrade_a_1)]
                .into_iter()
                .collect(),
        );

        let structure_b = Structure::new(upgrade_b, HashMap::new());

        let structure_c = Structure::new(upgrade_c, HashMap::new());

        vec![structure_a, structure_b, structure_c]
    }

    fn setup_test_structure_maps(
        structures: &[Structure],
    ) -> (HashMap<String, &Structure>, HashMap<Material, Vec<Upgrade>>) {
        let mut structure_map = HashMap::new();
        let mut output_map = HashMap::new();

        for structure in structures {
            structure_map.insert(structure.default_upgrade.name.clone(), structure);
            fill_upgrade_output_map(&structure.default_upgrade, &mut output_map);
            for upgrade in structure.upgrades.values() {
                fill_upgrade_output_map(upgrade, &mut output_map);
            }
        }

        (structure_map, output_map)
    }

    fn fill_upgrade_output_map(
        upgrade: &Upgrade,
        output_map: &mut HashMap<Material, Vec<Upgrade>>,
    ) {
        for production_channel in &upgrade.production_channels {
            for output in &production_channel.outputs {
                let output_entry = output_map.entry(output.material).or_default();
                output_entry.push(upgrade.clone());
            }
        }
    }

    #[test]
    fn test_calc_factory_reqs_multi_choice() {
        let structures = build_structures();
        let (structure_map, output_map) = setup_test_structure_maps(&structures);

        let rg = ResourceGraph::new(&structure_map, &output_map);

        let outputs = vec![(Material::Coke, 10)].into_iter().collect();
        let trees = rg.calculate_factory_requirements(outputs, HashSet::new());
        let reqs = rg.factory_requirements_from_trees(&trees, HashSet::new());

        let buildings = vec![FactoryRequirementsBuilding {
            building: "upgrade_a".to_string(),
            upgrade: Some("upgrade_a_1".to_string()),
            count: 5.0,
        }];

        let build_cost = vec![(Material::BasicMaterials, 10)].into_iter().collect();
        let inputs = vec![(Material::Coal, 5.0)].into_iter().collect();
        let expected_reqs = FactoryRequirements {
            buildings,
            power: 5.0,
            build_cost,
            inputs,
        };

        assert_eq!(reqs, expected_reqs);
    }

    #[test]
    fn test_calc_factory_reqs_multi_choice_activate_node() {
        let structures = build_structures();
        let (structure_map, output_map) = setup_test_structure_maps(&structures);

        let rg = ResourceGraph::new(&structure_map, &output_map);

        let outputs = vec![(Material::Coke, 10)].into_iter().collect();
        let mut trees = rg.calculate_factory_requirements(outputs, HashSet::new());

        let mut inactive_node_id = None;
        for node_id in trees[0].roots.as_ref().unwrap() {
            let node = trees[0].arena.get(*node_id).unwrap().get();
            if !node.active {
                inactive_node_id = Some(*node_id);

                break;
            }
        }

        trees[0].activate_node(
            inactive_node_id.expect("There should be one inactive tree node in this test"),
        );

        let buildings = vec![FactoryRequirementsBuilding {
            building: "upgrade_a".to_string(),
            upgrade: None,
            count: 10.0,
        }];

        let build_cost = vec![(Material::BasicMaterials, 10)].into_iter().collect();
        let inputs = vec![(Material::Coal, 10.0)].into_iter().collect();
        let expected_reqs = FactoryRequirements {
            buildings,
            power: 10.0,
            build_cost,
            inputs,
        };

        let reqs = rg.factory_requirements_from_trees(&trees, HashSet::new());
        assert_eq!(reqs, expected_reqs);
    }

    #[test]
    fn test_calc_factory_reqs_user_inputs() {
        let structures = build_structures();
        let (structure_map, output_map) = setup_test_structure_maps(&structures);

        let rg = ResourceGraph::new(&structure_map, &output_map);

        let inputs: HashSet<Material> = vec![Material::Components].into_iter().collect();
        let outputs = vec![(Material::Coke, 10)].into_iter().collect();
        let trees = rg.calculate_factory_requirements(outputs, inputs.clone());
        let reqs = rg.factory_requirements_from_trees(&trees, inputs);

        let buildings = vec![FactoryRequirementsBuilding {
            building: "upgrade_a".to_string(),
            upgrade: Some("upgrade_a_1".to_string()),
            count: 5.0,
        }];

        let build_cost = vec![(Material::BasicMaterials, 10)].into_iter().collect();
        let inputs = vec![(Material::Coal, 5.0)].into_iter().collect();
        let expected_reqs = FactoryRequirements {
            buildings,
            power: 5.0,
            build_cost,
            inputs,
        };

        assert_eq!(reqs, expected_reqs);
    }

    #[test]
    fn test_calc_factory_reqs_multiple_outputs() {
        let structures = build_structures();
        let (structure_map, output_map) = setup_test_structure_maps(&structures);

        let rg = ResourceGraph::new(&structure_map, &output_map);

        let inputs: HashSet<Material> = vec![Material::Components].into_iter().collect();
        let outputs = vec![(Material::Coke, 10), (Material::Rocket4CFire, 1)]
            .into_iter()
            .collect();
        let trees = rg.calculate_factory_requirements(outputs, inputs.clone());
        let reqs = rg.factory_requirements_from_trees(&trees, inputs);

        let buildings = vec![
            FactoryRequirementsBuilding {
                building: "upgrade_a".to_string(),
                upgrade: Some("upgrade_a_1".to_string()),
                count: 5.0,
            },
            FactoryRequirementsBuilding {
                building: "upgrade_b".to_string(),
                upgrade: None,
                count: 1.0,
            },
        ];

        let build_cost = vec![(Material::BasicMaterials, 11)].into_iter().collect();
        let inputs = vec![(Material::Coal, 5.0), (Material::Components, 1.0)]
            .into_iter()
            .collect();
        let expected_reqs = FactoryRequirements {
            buildings,
            power: 6.0,
            build_cost,
            inputs,
        };

        assert_eq!(reqs, expected_reqs);
    }

    #[test]
    fn test_calc_factory_reqs_multiple_outputs_same_structure() {
        let structures = build_structures();
        let (structure_map, output_map) = setup_test_structure_maps(&structures);

        let rg = ResourceGraph::new(&structure_map, &output_map);

        let inputs: HashSet<Material> = vec![Material::Components].into_iter().collect();
        let outputs = vec![
            (Material::Rocket3CHighExplosive, 1),
            (Material::Rocket4CFire, 1),
        ]
        .into_iter()
        .collect();
        let trees = rg.calculate_factory_requirements(outputs, inputs.clone());
        let reqs = rg.factory_requirements_from_trees(&trees, inputs);

        let buildings = vec![FactoryRequirementsBuilding {
            building: "upgrade_b".to_string(),
            upgrade: None,
            count: 2.0,
        }];

        let build_cost = vec![(Material::BasicMaterials, 2)].into_iter().collect();
        let inputs = vec![(Material::Components, 2.0)].into_iter().collect();
        let expected_reqs = FactoryRequirements {
            buildings,
            power: 2.0,
            build_cost,
            inputs,
        };

        assert_eq!(reqs, expected_reqs);
    }

    #[test]
    fn test_calc_factory_reqs_multiple_levels() {
        let structures = build_structures();
        let (structure_map, output_map) = setup_test_structure_maps(&structures);

        let rg = ResourceGraph::new(&structure_map, &output_map);

        let outputs = vec![(Material::ConcreteMaterials, 1)].into_iter().collect();
        let trees = rg.calculate_factory_requirements(outputs, HashSet::new());
        let reqs = rg.factory_requirements_from_trees(&trees, HashSet::new());

        let buildings = vec![
            FactoryRequirementsBuilding {
                building: "upgrade_a".to_string(),
                upgrade: Some("upgrade_a_1".to_string()),
                count: 0.5,
            },
            FactoryRequirementsBuilding {
                building: "upgrade_c".to_string(),
                upgrade: None,
                count: 1.0,
            },
        ];

        let build_cost = vec![(Material::BasicMaterials, 3)].into_iter().collect();
        let inputs = vec![(Material::Coal, 0.5)].into_iter().collect();
        let expected_reqs = FactoryRequirements {
            buildings,
            power: 2.0,
            build_cost,
            inputs,
        };

        assert_eq!(reqs, expected_reqs);
    }
}
