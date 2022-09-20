use std::collections::{HashMap, VecDeque};

use foxhole_production_calculator_types::Material::{self, *};
use foxhole_production_calculator_types::{
    BuildCost, Input, Output, ProductionChannel, Structure, Upgrade,
};

include!(concat!(env!("OUT_DIR"), "/structures.rs"));

#[derive(Debug, PartialEq)]
pub struct FactoryRequirements {
    pub buildings: HashMap<String, f32>,
    pub power: f32,
    pub build_cost: HashMap<Material, u64>,
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
        output: Material,
        rate: u64,
    ) -> FactoryRequirements {
        let mut buildings = HashMap::new();
        let mut power = 0.0;
        let mut build_costs = HashMap::new();

        let mut stack = Vec::new();
        stack.push((output, rate as f32));
        while let Some((current_input, current_rate)) = stack.pop() {
            let upgrades = self.upgrade_map.get(&current_input);

            if let Some(upgrades) = upgrades {
                let mut building_count = 0.0f32;
                for upgrade in upgrades {
                    for production_channel in &upgrade.production_channels {
                        // FIXME: This sucks, change outputs to be a map
                        for output in &production_channel.outputs {
                            if current_input == output.material {
                                building_count = current_rate as f32
                                    / production_channel.hourly_rate(output.value);

                                let building_name = upgrade
                                    .parent
                                    .clone()
                                    .unwrap_or_else(|| upgrade.name.clone());
                                let entry: &mut f32 = buildings.entry(building_name).or_default();
                                *entry += building_count;

                                break;
                            }
                        }

                        for input in &production_channel.inputs {
                            stack.push((
                                input.material,
                                production_channel.hourly_rate(input.value) * building_count,
                            ));
                        }
                    }
                }
            }
        }

        for (building, count) in &buildings {
            let structure = self
                .structure_map
                .get(building)
                .expect("Structure should exist");

            // FIXME: Actually look through production channels
            let production_channel = &structure.default_upgrade.production_channels[0];
            power += production_channel.power * count.ceil();
            for build_cost in &structure.default_upgrade.build_costs {
                let entry = build_costs.entry(build_cost.material).or_default();

                *entry += build_cost.cost * count.ceil() as u64;
            }
        }

        FactoryRequirements {
            buildings,
            power,
            build_cost: build_costs,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use foxhole_production_calculator_types::BuildCost;

    fn build_structures() -> Vec<Structure> {
        vec![
            Structure::new(
                "structure_a".to_string(),
                1,
                1,
                vec![BuildCost::new(Material::BasicMaterials, 1)],
                vec![Input::new(Material::Salvage, 1)],
                vec![Output::new(Material::ConstructionMaterials, 1)],
                vec![],
            ),
            Structure::new(
                "structure_b".to_string(),
                1,
                1,
                vec![BuildCost::new(Material::BasicMaterials, 1)],
                vec![Input::new(Material::Coal, 1)],
                vec![Output::new(Material::Coke, 1)],
                vec![],
            ),
            Structure::new(
                "structure_c".to_string(),
                1,
                1,
                vec![
                    BuildCost::new(Material::BasicMaterials, 1),
                    BuildCost::new(Material::ConstructionMaterials, 1),
                ],
                vec![
                    Input::new(Material::ConstructionMaterials, 1),
                    Input::new(Material::Coke, 1),
                ],
                vec![Output::new(Material::ProcessedConstructionMaterials, 1)],
                vec![],
            ),
        ]
    }

    fn setup_test_structure_maps(
        structures: &[Structure],
    ) -> (HashMap<String, &Structure>, HashMap<Material, &Structure>) {
        let mut structure_map = HashMap::new();
        let mut output_map = HashMap::new();

        for structure in structures {
            structure_map.insert(structure.name.clone(), structure);
            for output in &structure.outputs {
                output_map.insert(output.material, structure);
            }
        }

        (structure_map, output_map)
    }

    #[test]
    fn test_calc_factory_reqs() {
        let structures = build_structures();
        let (structure_map, output_map) = setup_test_structure_maps(&structures);

        let rg = ResourceGraph::new(&structure_map, &output_map);

        let reqs =
            rg.calculate_factory_requirements(Material::ProcessedConstructionMaterials, 36000);

        let buildings = vec![
            ("structure_b".to_string(), 10.0),
            ("structure_c".to_string(), 10.0),
            ("structure_a".to_string(), 10.0),
        ]
        .into_iter()
        .collect::<HashMap<String, f32>>();

        let build_cost = vec![
            (Material::BasicMaterials, 30),
            (Material::ConstructionMaterials, 10),
        ]
        .into_iter()
        .collect::<HashMap<Material, u64>>();

        let expected_reqs = FactoryRequirements {
            buildings,
            power: 30,
            build_cost,
        };

        assert_eq!(reqs, expected_reqs);
    }
}
