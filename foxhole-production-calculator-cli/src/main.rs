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

    /// Optional argument specifying inputs that will be brought in externally from the factory.
    /// Multiple values can be input with comma seperators.
    #[clap(short, long, value_parser, use_value_delimiter = true)]
    user_inputs: Option<Vec<Material>>,
}

fn main() {
    let args = Args::parse();

    let rg = ResourceGraph::default();

    let user_inputs = args.user_inputs.unwrap_or_default();
    let output = vec![(args.material, args.rate)].into_iter().collect();
    let trees =
        rg.calculate_factory_requirements(output, user_inputs.clone().into_iter().collect());
    let reqs = rg.factory_requirements_from_trees(&trees, user_inputs.into_iter().collect());

    println!("{}", serde_json::to_string_pretty(&reqs).unwrap());
}
