use clap::Parser;
use foxhole_production_calculator::ResourceGraph;
use foxhole_production_calculator_types::Material;

/// Command utility for calculating the most efficient factory configuration for
/// a target output.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Specifies the output material for the factory.
    #[clap(arg_enum, value_parser)]
    material: Material,

    /// Specifies the desired rate of output for the given material. [Unit/Hour]
    #[clap(value_parser)]
    rate: u64,
}

fn main() {
    let args = Args::parse();

    let rg = ResourceGraph::default();
    let reqs = rg.calculate_factory_requirements(args.material, args.rate);

    println!("{}", serde_json::to_string_pretty(&reqs).unwrap());
}
