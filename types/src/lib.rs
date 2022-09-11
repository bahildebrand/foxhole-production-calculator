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
    /// Name of the structure.
    pub name: String,
    /// Power required to run the structure in MW.
    pub power: u64,
    /// Rate of production in seconds.
    pub rate: u64,
    pub build_costs: Vec<BuildCost>,
    pub inputs: Vec<Input>,
    pub outputs: Vec<Output>,
}

impl Structure {
    const SECONDS_PER_HOUR: f32 = 60.0 * 60.0;

    pub fn new(
        name: String,
        power: u64,
        rate: u64,
        build_costs: Vec<BuildCost>,
        inputs: Vec<Input>,
        outputs: Vec<Output>,
    ) -> Self {
        Self {
            name,
            power,
            rate,
            build_costs,
            inputs,
            outputs,
        }
    }

    pub fn hourly_rate(&self, rate: u64) -> f32 {
        let ticks_per_hour = Self::SECONDS_PER_HOUR / self.rate as f32;

        ticks_per_hour * rate as f32
    }
}

impl fmt::Debug for Structure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Structure")
            .field("name", &format_args!("String::from({:?})", &self.name))
            .field("power", &self.power)
            .field("rate", &self.rate)
            .field(
                "build_costs",
                &format_args!("{:?}.to_vec()", &self.build_costs),
            )
            .field("inputs", &format_args!("{:#?}.to_vec()", &self.inputs))
            .field("outputs", &format_args!("{:#?}.to_vec()", &self.outputs))
            .finish()
    }
}
