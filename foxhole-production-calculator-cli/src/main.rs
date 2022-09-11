use calculator::ResourceGraph;
use clap::Parser;
use types::Material;

/// Command utility for calculating the most efficient factory configuration for
/// a target output.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Specifies the output material for the factory.
    #[clap(arg_enum, value_parser)]
    material: Material,

    /// Specifies the desired rate of output for the given material.
    #[clap(value_parser)]
    rate: u64,
}

fn main() {
    let args = Args::parse();

    let rg = ResourceGraph::default();

    println!(
        "{:#?}",
        rg.calculate_factory_requirements(args.material, args.rate)
    );
}
