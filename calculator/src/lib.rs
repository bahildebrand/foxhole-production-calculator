use std::collections::{HashMap, VecDeque};

use types::Material::{self, *};
use types::{BuildCost, Input, Output, Structure};

include!(concat!(env!("OUT_DIR"), "/structures.rs"));

#[derive(Debug)]
pub struct FactoryRequirements {
    pub buildings: HashMap<String, f32>,
    pub power: u64,
    pub build_cost: HashMap<Material, u64>,
}

pub struct ResourceGraph {
    structure_map: &'static HashMap<String, &'static Structure>,
    output_map: &'static HashMap<Material, &'static Structure>,
}

impl Default for ResourceGraph {
    fn default() -> Self {
        Self {
            structure_map: &*STRUCTURE_MAP,
            output_map: &*OUTPUT_MAP,
        }
    }
}

impl ResourceGraph {
    pub fn calculate_factory_requirements(
        &self,
        output: Material,
        rate: u64,
    ) -> FactoryRequirements {
        let mut buildings = HashMap::new();
        let mut power = 0;
        let mut build_costs = HashMap::new();

        let mut queue = VecDeque::new();
        queue.push_back((output, rate));
        while let Some((current_input, current_rate)) = queue.pop_front() {
            let structure = self.output_map.get(&current_input);

            if let Some(structure) = structure {
                // FIXME: This sucks, change outputs to be a map
                for output in &structure.outputs {
                    if current_input == output.material {
                        let building_count = current_rate as f32 / output.value as f32;

                        let entry = buildings.entry(structure.name.clone()).or_default();
                        *entry += building_count;

                        break;
                    }
                }

                for input in &structure.inputs {
                    queue.push_back((input.material, input.value));
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

    #[test]
    fn test_calc_factory_reqs() {
        let rg = ResourceGraph::default();

        let reqs = rg.calculate_factory_requirements(Material::ProcessedConstructionMaterials, 10);
        println!("{reqs:?}");
    }
}
