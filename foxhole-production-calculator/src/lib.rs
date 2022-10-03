use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};

use foxhole_production_calculator_types::Material::{self, *};
use foxhole_production_calculator_types::{
    BuildCost, Input, Output, ProductionChannel, Structure, Upgrade,
};
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

#[derive(Debug, Clone, Serialize)]
pub struct FactoryRequirementsBuilding {
    pub building: String,
    pub upgrade: Option<String>,
    pub count: f32,
}

impl PartialEq for FactoryRequirementsBuilding {
    fn eq(&self, other: &Self) -> bool {
        self.building == other.building && self.upgrade == other.upgrade
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
    ) -> FactoryRequirements {
        let mut buildings = HashMap::new();
        let mut stack = Vec::new();
        let mut inputs = HashMap::new();

        for (output, rate) in outputs.into_iter() {
            stack.push((output, rate as f32));
        }
        while let Some((current_input, current_rate)) = stack.pop() {
            if let Some(upgrades) = self.upgrade_map.get(&current_input) {
                if !user_inputs.contains(&current_input) {
                    self.calculate_building_counts(
                        upgrades,
                        &mut buildings,
                        current_input,
                        current_rate,
                        &mut stack,
                    );
                } else {
                    self.calculate_inputs(current_input, current_rate, &mut inputs);
                }
            } else {
                self.calculate_inputs(current_input, current_rate, &mut inputs);
            }
        }

        let mut build_costs = HashMap::new();
        let mut power = 0.0;
        for (structure_key, count) in &buildings {
            if let Some(parent) = &structure_key.parent {
                // Non-default upgrade case
                let structure = self
                    .structure_map
                    .get(parent)
                    .expect("Structure should exist");
                let upgrade = structure
                    .upgrades
                    .get(&structure_key.upgrade)
                    .expect("Upgrade should exist");

                calculate_build_costs(&mut build_costs, &structure.default_upgrade, *count);
                calculate_build_costs(&mut build_costs, upgrade, *count);

                power += upgrade.production_channels[structure_key.prod_channel_idx].power
                    * count.ceil();
            } else {
                // Default upgrade case
                let structure = self
                    .structure_map
                    .get(&structure_key.upgrade)
                    .expect("Structure should exist");

                calculate_build_costs(&mut build_costs, &structure.default_upgrade, *count);

                let production_channel =
                    &structure.default_upgrade.production_channels[structure_key.prod_channel_idx];
                power += production_channel.power * count.ceil();
            }
        }

        // Sort here to avoid non-determinism in test and outputs.
        let buildings: Vec<FactoryRequirementsBuilding> =
            sorted(buildings.into_iter().map(|(building, count)| {
                if let Some(parent) = building.parent {
                    FactoryRequirementsBuilding {
                        building: parent,
                        upgrade: Some(building.upgrade),
                        count,
                    }
                } else {
                    FactoryRequirementsBuilding {
                        building: building.upgrade,
                        upgrade: None,
                        count,
                    }
                }
            }))
            .collect();

        FactoryRequirements {
            buildings,
            power,
            build_cost: build_costs,
            inputs,
        }
    }

    fn calculate_building_counts(
        &self,
        upgrades: &[Upgrade],
        buildings: &mut HashMap<StructureKey, f32>,
        current_input: Material,
        current_rate: f32,
        stack: &mut Vec<(Material, f32)>,
    ) {
        let mut highest_upgrade = None;
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

                        if let Some((highest_output_val, upgrade)) = &mut highest_upgrade {
                            if output.value > *highest_output_val {
                                *highest_output_val = output.value;
                                *upgrade = structure_key;
                            }
                        } else {
                            highest_upgrade = Some((output.value, structure_key));
                        }

                        break;
                    }
                }
            }
        }

        let (_, structure_key) = highest_upgrade.expect("Upgrade should exist");
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
        for input in &production_channel.inputs {
            let building_count = current_rate as f32 / production_channel.hourly_rate(output_value);

            let entry: &mut f32 = buildings.entry(structure_key.clone()).or_default();
            *entry += building_count;

            stack.push((
                input.material,
                production_channel.hourly_rate(input.value) * building_count,
            ));
        }
    }

    fn calculate_inputs(
        &self,
        current_input: Material,
        current_rate: f32,
        inputs: &mut HashMap<Material, f32>,
    ) {
        // If the material can't be find in output map it cannot be created by a player facility.
        // Mark how much is needed per hour for later input
        let entry = inputs.entry(current_input).or_default();

        *entry += current_rate;
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
            vec![ProductionChannel {
                power: 1.0,
                rate: 3600,
                inputs: vec![Input::new(Material::Components, 1)],
                outputs: vec![Output::new(Material::Rocket4CFire, 1)],
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

        vec![structure_a, structure_b]
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

        let outputs = vec![(Material::Rocket4CFire, 10)].into_iter().collect();
        let reqs = rg.calculate_factory_requirements(outputs, HashSet::new());

        let buildings = vec![FactoryRequirementsBuilding {
            building: "upgrade_b".to_string(),
            upgrade: None,
            count: 10.0,
        }];

        let build_cost = vec![(Material::BasicMaterials, 10)].into_iter().collect();
        let inputs = vec![(Material::Components, 10.0)].into_iter().collect();
        let expected_reqs = FactoryRequirements {
            buildings,
            power: 10.0,
            build_cost,
            inputs,
        };

        assert_eq!(reqs, expected_reqs);
    }

    #[test]
    fn test_calc_factory_reqs_user_inputs() {
        let structures = build_structures();
        let (structure_map, output_map) = setup_test_structure_maps(&structures);

        let rg = ResourceGraph::new(&structure_map, &output_map);

        let inputs = vec![Material::Components].into_iter().collect();
        let outputs = vec![(Material::Coke, 10)].into_iter().collect();
        let reqs = rg.calculate_factory_requirements(outputs, inputs);

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

        let inputs = vec![Material::Components].into_iter().collect();
        let outputs = vec![(Material::Coke, 10), (Material::Rocket4CFire, 1)]
            .into_iter()
            .collect();
        let reqs = rg.calculate_factory_requirements(outputs, inputs);

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
}
