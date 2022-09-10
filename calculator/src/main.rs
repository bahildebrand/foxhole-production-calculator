use types::Material::{self, *};
use types::{BuildCost, Input, Output, Structure};

include!(concat!(env!("OUT_DIR"), "/structures.rs"));

fn main() {
    println!("{:?}", OUTPUT_MAP.get(&Material::Napalm));
}
