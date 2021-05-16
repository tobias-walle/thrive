use std::{env, fs};

use schemars::schema_for;

fn main() {
    color_backtrace::install();
    generate_schemas();
}

macro_rules! save_schema_for {
    ($type: ty, $path: expr) => {
        println!("Save schema for {:?} to {:?}", stringify!($type), $path);
        let schema = schema_for!($type);
        let schema = serde_json::to_string_pretty(&schema).unwrap();
        fs::write($path, schema).unwrap();
    };
}

fn generate_schemas() {
    let current_dir = env::current_dir().unwrap();
    let schemas_dir = current_dir.join("schemas");
    println!("Ensure output directory {:?}", schemas_dir);
    fs::remove_dir_all(&schemas_dir).unwrap();
    fs::create_dir_all(&schemas_dir).unwrap();
    save_schema_for!(
        thrive_core::command::Command,
        schemas_dir.join("command.json")
    );
    save_schema_for!(thrive_core::event::Event, schemas_dir.join("event.json"));
}
