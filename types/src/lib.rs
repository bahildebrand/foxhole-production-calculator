use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Material {
    BasicMaterials,
    Scrap,
    ConstructionMaterials,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BuildCost {
    pub material: Material,
    pub cost: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Input {
    pub material: Material,
    pub value: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Output {
    pub material: Material,
    pub value: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Structure {
    pub name: String,
    pub power: u64,
    pub build_costs: Vec<BuildCost>,
    pub inputs: Vec<Input>,
    pub outputs: Vec<Output>,
}
