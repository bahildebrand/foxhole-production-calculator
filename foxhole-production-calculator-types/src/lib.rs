use std::{collections::HashMap, hash::Hash};

use clap::ValueEnum;
use genco::{
    prelude::*,
    quote_in,
    tokens::{static_literal, FormatInto, ItemStr},
    Tokens,
};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, EnumString};

#[derive(
    Debug,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    Hash,
    Clone,
    Copy,
    ValueEnum,
    PartialOrd,
    Ord,
    EnumIter,
    Display,
    EnumString,
)]
#[strum(serialize_all = "title_case")]
pub enum Material {
    BasicMaterials,
    Salvage,
    ConstructionMaterials,
    ProcessedConstructionMaterials,
    Oil,
    Petrol,
    Coal,
    Coke,
    ExplosiveMaterials,
    HeavyExplosiveMaterials,
    FlameAmmo,
    Components,
    Water,
    HeavyOil,
    EnrichedOil,
    Sulfur,
    SteelConstructionMaterials,
    ConcreteMaterials,
    Pipe,
    AssemblyMaterialsI,
    AssemblyMaterialsII,
    AssemblyMaterialsIII,
    AssemblyMaterialsIV,
    AssemblyMaterialsV,
    MetalBeam,
    SandBag,
    BarbedWire,
    #[strum(serialize = "Rocket 3C High Explosive")]
    Rocket3CHighExplosive,
    #[strum(serialize = "Rocket 4C Fire")]
    Rocket4CFire,
    #[strum(serialize = "Shell 75mm")]
    Shell75MM,
    #[strum(serialize = "Shell 94.5mm")]
    Shell945MM,
    #[strum(serialize = "Shell 120mm")]
    Shell120MM,
    #[strum(serialize = "Shell 150mm")]
    Shell150MM,
    #[strum(serialize = "Shell 250mm")]
    Shell250MM,
    #[strum(serialize = "Shell 300mm")]
    Shell300MM,
    DamagedComponents,
}

impl<L> FormatInto<L> for Material
where
    L: Lang,
{
    fn format_into(self, tokens: &mut Tokens<L>) {
        let out_str = format!("{:?}", self);

        let literal = ItemStr::Box(out_str.into());
        tokens.append(literal);
    }
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

impl<L> FormatInto<L> for BuildCost
where
    L: Lang,
{
    fn format_into(self, tokens: &mut Tokens<L>) {
        let Self { material, cost } = self;

        quote_in! { *tokens =>
            BuildCost {
                material: $material,
                cost: $cost
            }
        }
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

impl<L> FormatInto<L> for Input
where
    L: Lang,
{
    fn format_into(self, tokens: &mut Tokens<L>) {
        let Self { material, value } = self;

        quote_in! { *tokens =>
            Input {
                material: $material,
                value: $value,
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct Output {
    pub material: Material,
    pub value: u64,
}

impl Output {
    pub fn new(material: Material, value: u64) -> Self {
        Self { material, value }
    }
}

impl<L> FormatInto<L> for Output
where
    L: Lang,
{
    fn format_into(self, tokens: &mut Tokens<L>) {
        let Self { material, value } = self;

        quote_in! { *tokens =>
            Output {
                material: $material,
                value: $value,
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProductionChannel {
    /// Power required to run the structure in MW.
    pub power: f32,
    /// Rate of production in seconds.
    pub rate: u64,
    pub inputs: Vec<Input>,
    pub outputs: Vec<Output>,
}

impl<L> FormatInto<L> for ProductionChannel
where
    L: Lang,
{
    fn format_into(self, tokens: &mut Tokens<L>) {
        let Self {
            power,
            rate,
            inputs,
            outputs,
        } = self;
        let power_str = format!("{:.2}", power);

        quote_in! { *tokens =>
            ProductionChannel {
                power: $power_str,
                rate: $rate,
                inputs: vec![$(for input in inputs => $input,$[' '])],
                outputs: vec![$(for output in outputs => $output,$[' '])],
            }
        }
    }
}

impl ProductionChannel {
    const SECONDS_PER_HOUR: f32 = 60.0 * 60.0;

    pub fn hourly_rate(&self, rate: u64) -> f32 {
        let ticks_per_hour = Self::SECONDS_PER_HOUR / self.rate as f32;

        ticks_per_hour * rate as f32
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Upgrade {
    /// Name of the structure.
    pub name: String,
    pub build_costs: Vec<BuildCost>,
    pub production_channels: Vec<ProductionChannel>,
    pub parent: Option<String>,
}

impl Upgrade {
    pub fn new(
        name: String,
        build_costs: Vec<BuildCost>,
        production_channels: Vec<ProductionChannel>,
        parent: Option<String>,
    ) -> Self {
        Self {
            name,
            build_costs,
            production_channels,
            parent,
        }
    }
}

impl PartialEq for Upgrade {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.parent == other.parent
    }
}

impl Eq for Upgrade {}

impl Hash for Upgrade {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.parent.hash(state);
    }
}

impl<L> FormatInto<L> for Upgrade
where
    L: Lang,
{
    fn format_into(self, tokens: &mut Tokens<L>) {
        let Self {
            name,
            build_costs,
            production_channels,
            parent,
        } = self;

        quote_in! { *tokens =>
            Upgrade {
                name: $(quoted(name)).to_string(),
                build_costs: vec![$(for cost in build_costs => $cost,$[' '])],
                production_channels: vec![$(for channel in production_channels => $channel,$[' '])],
                parent: $(if parent.is_some() { Some($(quoted(parent)).to_string()) } else { None })
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Structure {
    pub default_upgrade: Upgrade,
    pub upgrades: HashMap<String, Upgrade>,
}

impl Structure {
    pub fn new(default_upgrade: Upgrade, upgrades: HashMap<String, Upgrade>) -> Self {
        Self {
            default_upgrade,
            upgrades,
        }
    }
}

impl<L> FormatInto<L> for Structure
where
    L: Lang,
{
    fn format_into(self, tokens: &mut Tokens<L>) {
        let default_upgrade = self.default_upgrade;
        // let upgrades: Vec<(String, Upgrade)> = self.upgrades.into_iter().collect();

        let mut upgrade_tokens = Tokens::new();
        upgrade_tokens.append(static_literal("vec!["));
        for (name, upgrade) in self.upgrades.into_iter() {
            quote_in! { upgrade_tokens =>
                ($(quoted(name)).to_string(), $upgrade),
            };
        }
        upgrade_tokens.append(static_literal("].into_iter().collect()"));

        quote_in! { *tokens =>
            Structure {
                default_upgrade: $default_upgrade,
                upgrades: $upgrade_tokens
            }
        }
    }
}
