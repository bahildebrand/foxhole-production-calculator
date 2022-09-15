use std::{collections::HashMap, fmt};

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
    Oil,
    Petrol,
    Coal,
    Coke,
    HeavyExplosiveMaterials,
    Napalm,
    Components,
    Water,
    HeavyOil,
    EnrichedOil,
    Sulfur,
    SteelConstructionMaterials,
    ConcreteMaterials,
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
pub struct ProductionChannel {
    /// Power required to run the structure in MW.
    pub power: f32,
    /// Rate of production in seconds.
    pub rate: u64,
    pub build_costs: Vec<BuildCost>,
    pub inputs: Vec<Input>,
    pub outputs: Vec<Output>,
}

// FIXME: This is a brain dead way to generate code. God help me.
impl fmt::Debug for ProductionChannel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ProductionChannel")
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

#[derive(Serialize, Deserialize, Clone)]
pub struct Upgrade {
    /// Name of the structure.
    pub name: String,
    pub production_channels: Vec<ProductionChannel>,
}

impl fmt::Debug for Upgrade {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Upgrade")
            .field("name", &format_args!("String::from({:?})", &self.name))
            .field(
                "production_channels",
                &format_args!("{:?}.to_vec()", &self.production_channels),
            )
            .finish()
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Structure {
    pub default_upgrade: Upgrade,
    pub upgrades: HashMap<String, Upgrade>,
}

impl Structure {
    const SECONDS_PER_HOUR: f32 = 60.0 * 60.0;

    pub fn new(default_upgrade: Upgrade, upgrades: HashMap<String, Upgrade>) -> Self {
        Self {
            default_upgrade,
            upgrades,
        }
    }

    pub fn hourly_rate(&self, rate: u64) -> f32 {
        // FIXME: The production_channel needs to be chosen dynamically
        let ticks_per_hour =
            Self::SECONDS_PER_HOUR / self.default_upgrade.production_channels[0].rate as f32;

        ticks_per_hour * rate as f32
    }
}

// This is also brain dead. God help me here too.
impl fmt::Debug for Structure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Structure")
            .field("default_upgrade", &self.default_upgrade)
            .field("upgrades", &format_args!("{:#?}.to_vec()", self.upgrades))
            .finish()
    }
}
