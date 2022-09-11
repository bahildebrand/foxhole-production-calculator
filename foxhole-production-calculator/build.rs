use std::env;
use std::fmt::Write;

use foxhole_production_calculator_types::Structure;

fn main() {
    println!("cargo:rerun-if-changed=structures/");
    println!("cargo:rerun-if-changed=build.rs");

    let out_dir = env::var("OUT_DIR").unwrap();

    let mut out_str = String::from("lazy_static::lazy_static!{\n");
    let mut structure_map_str = String::from(
        "\tstatic ref STRUCTURE_MAP: std::collections::HashMap<String, &'static Structure> = vec![\n",
    );
    let mut output_map_str = String::from(
        "\tstatic ref OUTPUT_MAP: std::collections::HashMap<Material, &'static Structure> = vec![\n",
    );

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
        let const_string = format!(
            "\tstatic ref {}: Structure = {:#?};\n\n",
            struct_name.clone(),
            structure
        );

        out_str.push_str(&const_string);

        // Add structure mapping entry
        writeln!(
            &mut structure_map_str,
            "\t\t(\"{}\".to_string(), &*{}),",
            structure.name.clone(),
            struct_name
        )
        .expect("Failed to write to code generation string");

        // Add output mapping entry
        for output in structure.outputs {
            writeln!(
                &mut output_map_str,
                "\t\t({:?}, &*{}),",
                output.material, struct_name
            )
            .expect("Failed to write to code generation string");
        }
    }

    structure_map_str.push_str("\t].into_iter().collect();\n\n");
    output_map_str.push_str("\t].into_iter().collect();\n");

    out_str.push_str(&structure_map_str);
    out_str.push_str(&output_map_str);
    out_str.push_str("\n}");
    std::fs::write(format!("{out_dir}/structures.rs"), out_str).unwrap();
}
