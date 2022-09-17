use std::fs::File;
use std::{collections::HashMap, env};

use foxhole_production_calculator_types::Structure;
use genco::tokens::static_literal;
use genco::{fmt, lang, prelude::*};

fn main() {
    println!("cargo:rerun-if-changed=structures/");
    println!("cargo:rerun-if-changed=build.rs");

    let out_dir = env::var("OUT_DIR").unwrap();

    let mut structures = HashMap::new();
    // TODO: Actually handle these errors
    for entry in std::fs::read_dir("structures").unwrap() {
        let entry = entry.unwrap();
        if !entry.file_type().unwrap().is_file() {
            // TODO: Support directories
            continue;
        }

        let file_name = String::from(entry.file_name().to_str().unwrap());
        let toml_string = std::fs::read_to_string(format!("structures/{}", file_name)).unwrap();

        let structure: Structure = toml::from_str(&toml_string).unwrap();

        let struct_name = file_name.split('.').collect::<Vec<_>>()[0].to_uppercase();

        structures.insert(struct_name.clone(), structure.clone());
    }

    let lazy_static = rust::import("lazy_static", "lazy_static");

    // Begin lazy static scope
    let mut tokens: Tokens<lang::Rust> = quote! {
        $lazy_static!$[' ']
    };
    tokens.append(static_literal("{"));
    tokens.push();

    // Build named structures
    for (name, structure) in structures.clone().into_iter() {
        let structure_import = rust::import("foxhole_production_calculator_types", "Structure");
        quote_in! { tokens =>
            static ref $name: $structure_import = $structure;
        };

        tokens.push();
    }

    // Build output map
    let hashmap = rust::import("std::collections", "HashMap");
    let structure_import = rust::import("foxhole_production_calculator_types", "Structure");
    quote_in! { tokens =>
        static ref OUTPUT_MAP: $hashmap<Material, &'static $structure_import> =$[' ']
    };
    tokens.append(static_literal("vec!["));
    tokens.push();
    for (name, structure) in structures.clone().into_iter() {
        // FIXME: Need to account for all production paths
        let output_mat = structure.default_upgrade.production_channels[0].outputs[0].material;

        quote_in! { tokens =>
            ($output_mat, &*$name),
        }
    }
    tokens.append(static_literal("].into_iter().collect();"));
    tokens.push();

    // Build structure map
    let hashmap = rust::import("std::collections", "HashMap");
    let structure_import = rust::import("foxhole_production_calculator_types", "Structure");
    quote_in! { tokens =>
        static ref STRUCTURE_MAP: $hashmap<String, &'static $structure_import> = $[' ']
    };
    tokens.append(static_literal("vec!["));
    tokens.push();
    for (name, structure) in structures.clone().into_iter() {
        let name_str = structure.default_upgrade.name;

        quote_in! { tokens =>
            ($(quoted(name_str)).to_string(), &*$name),
        }
    }
    tokens.append(static_literal("].into_iter().collect();"));
    tokens.push();

    // End lazy static scope
    tokens.append(static_literal("}"));

    // Open file and config code generator
    let out_file = File::create(format!("{out_dir}/structures.rs")).unwrap();
    let mut writer = fmt::IoWriter::new(out_file);
    let fmt = fmt::Config::from_lang::<Rust>().with_indentation(fmt::Indentation::Space(4));
    let config = rust::Config::default().with_default_import(rust::ImportMode::Qualified);

    // Write file
    tokens
        .format_file(&mut writer.as_formatter(&fmt), &config)
        .unwrap();
}
