use std::collections::{HashMap, VecDeque};

use types::Material::{self, *};
use types::{BuildCost, Input, Output, Structure};

include!(concat!(env!("OUT_DIR"), "/structures.rs"));

#[derive(Debug, PartialEq)]
pub struct FactoryRequirements {
    pub buildings: HashMap<String, f32>,
    pub power: u64,
    pub build_cost: HashMap<Material, u64>,
}

pub struct ResourceGraph<'a> {
    structure_map: &'a HashMap<String, &'a Structure>,
    output_map: &'a HashMap<Material, &'a Structure>,
}

impl<'a> Default for ResourceGraph<'a> {
    fn default() -> Self {
        Self {
            structure_map: &*STRUCTURE_MAP,
            output_map: &*OUTPUT_MAP,
        }
    }
}

impl<'a> ResourceGraph<'a> {
    #[cfg(test)]
    fn new(
        structure_map: &'a HashMap<String, &'a Structure>,
        output_map: &'a HashMap<Material, &'a Structure>,
    ) -> Self {
        Self {
            structure_map,
            output_map,
        }
    }

    pub fn calculate_factory_requirements(
        &self,
        output: Material,
        rate: u64,
    ) -> FactoryRequirements {
        let mut buildings = HashMap::new();
        let mut power = 0;
        let mut build_costs = HashMap::new();

        let mut queue = VecDeque::new();
        queue.push_back((output, rate as f32));
        while let Some((current_input, current_rate)) = queue.pop_front() {
            let structure = self.output_map.get(&current_input);

            if let Some(structure) = structure {
                // FIXME: This sucks, change outputs to be a map
                let mut building_count = 0.0f32;
                for output in &structure.outputs {
                    if current_input == output.material {
                        building_count = current_rate as f32 / output.value as f32;

                        let entry = buildings.entry(structure.name.clone()).or_default();
                        *entry += building_count;

                        break;
                    }
                }

                for input in &structure.inputs {
                    queue.push_back((input.material, input.value as f32 * building_count));
                }
            }
        }

        for (building, count) in &buildings {
            let structure = self
                .structure_map
                .get(building)
                .expect("Structure should exist");

            power += (structure.power as f32 * count).ceil() as u64;
            for build_cost in &structure.build_costs {
                let entry = build_costs.entry(build_cost.material).or_default();

                *entry += (build_cost.cost as f32 * count).ceil() as u64;
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
    use types::BuildCost;

    fn build_structures() -> Vec<Structure> {
        vec![
            Structure::new(
                "structure_a".to_string(),
                1,
                vec![BuildCost::new(Material::BasicMaterials, 1)],
                vec![Input::new(Material::Scrap, 1)],
                vec![Output::new(Material::ConstructionMaterials, 1)],
            ),
            Structure::new(
                "structure_b".to_string(),
                1,
                vec![BuildCost::new(Material::BasicMaterials, 1)],
                vec![Input::new(Material::Coal, 1)],
                vec![Output::new(Material::Coke, 1)],
            ),
            Structure::new(
                "structure_c".to_string(),
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

        let reqs = rg.calculate_factory_requirements(Material::ProcessedConstructionMaterials, 10);

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
