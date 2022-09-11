use std::fmt;

use clap::ValueEnum;
use serde::{Deserialize, Serialize};

#[derive(
    Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Copy, ValueEnum, PartialOrd, Ord,
)]
pub enum Material {
    BasicMaterials,
    Salvage,
    ConstructionMaterials,
    ProcessedConstructionMaterials,
    RefinedOil,
    Petrol,
    Coal,
    Coke,
    HeavyExplosiveMaterials,
    Napalm,
    Components,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BuildCost {
    pub material: Material,
    pub cost: u64,
}

impl BuildCost {
    pub fn new(material: Material, cost: u64) -> Self {
        Self { material, cost }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Input {
    pub material: Material,
    pub value: u64,
}

impl Input {
    pub fn new(material: Material, value: u64) -> Self {
        Self { material, value }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Output {
    pub material: Material,
    pub value: u64,
}

impl Output {
    pub fn new(material: Material, value: u64) -> Self {
        Self { material, value }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Structure {
    pub name: String,
    pub power: u64,
    pub build_costs: Vec<BuildCost>,
    pub inputs: Vec<Input>,
    pub outputs: Vec<Output>,
}

impl Structure {
    pub fn new(
        name: String,
        power: u64,
        build_costs: Vec<BuildCost>,
        inputs: Vec<Input>,
        outputs: Vec<Output>,
    ) -> Self {
        Self {
            name,
            power,
            build_costs,
            inputs,
            outputs,
        }
    }
}

impl fmt::Debug for Structure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Structure")
            .field("name", &format_args!("String::from({:?})", &self.name))
            .field("power", &self.power)
            .field(
                "build_costs",
                &format_args!("{:?}.to_vec()", &self.build_costs),
            )
            .field("inputs", &format_args!("{:#?}.to_vec()", &self.inputs))
            .field("outputs", &format_args!("{:#?}.to_vec()", &self.outputs))
            .finish()
    }
}
