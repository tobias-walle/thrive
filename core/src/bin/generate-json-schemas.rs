use std::{fs, path::Path};

use clap::{App, Arg};
use schemars::schema_for;

fn main() {
    color_backtrace::install();
    let args = App::new("generate-json-schema")
        .arg(Arg::from_usage(
            "<schemas-dir> 'In which directory should the schema be generated'",
        ))
        .get_matches();
    generate_schemas(args.value_of("schemas-dir").unwrap());
}

macro_rules! save_schema_for {
    ($type: ty, $path: expr) => {
        println!("Save schema for {:?} to {:?}", stringify!($type), $path);
        let schema = schema_for!($type);
        let schema = serde_json::to_string_pretty(&schema).unwrap();
        fs::write($path, schema).unwrap();
    };
}

fn generate_schemas(schemas_dir: &str) {
    let schemas_dir: &Path = schemas_dir.as_ref();
    println!("Ensure output directory {:?}", schemas_dir);
    let _ = fs::remove_dir_all(&schemas_dir);
    fs::create_dir_all(&schemas_dir).unwrap();
    save_schema_for!(
        thrive_core::command::Command,
        schemas_dir.join("command.json")
    );
    save_schema_for!(thrive_core::event::Event, schemas_dir.join("event.json"));
}
