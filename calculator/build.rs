use std::fmt::Write;

use types::Structure;

fn main() {
    println!("cargo:rerun-if-changed=structures/");
    println!("cargo:rerun-if-changed=build.rs");

    use std::env;
    let out_dir = env::var("OUT_DIR").unwrap();

    let mut out_str = String::from("use types::Material::*;\n\n");
    let mut map_str = String::from(
        "lazy_static::lazy_static!{\n\tstatic ref MAP: std::collections::HashMap<String, Structure> = vec![\n",
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
            "const {}: Structure = {:#?};\n\n",
            struct_name.clone(),
            structure
        );

        out_str.push_str(&const_string);
        writeln!(
            &mut map_str,
            "\t\t(\"{}\".to_string(), &{}),",
            structure.name.clone(),
            struct_name
        )
        .expect("Failed to write to cod generation string");
    }

    map_str.push_str("\t].into_iter().collect();\n}");

    // out_str.push_str("\n");
    out_str.push_str(&map_str);
    std::fs::write(format!("{out_dir}/structures.rs"), out_str).unwrap();
}
