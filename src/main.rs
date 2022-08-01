use serde::Deserialize;
use std::fs;
use std::process::Command;
use std::string::String;
use toml;

use chroma::utils::str::{to_macro_name, quote};

#[derive(Deserialize, Debug)]
struct Package {
    name: String,
    version: String,
    edition: String,
}

#[derive(Deserialize, Debug)]
struct Bin {
    name: String,
    path: String,
}

#[derive(Deserialize, Debug)]
struct Config {
    package: Package,
    bin: Vec<Bin>,
}


fn get_definitions(cfg: &Config) -> Vec<(String, String)> {
    let name = to_macro_name(cfg.package.name.as_str());

    let tag = |x| name.clone() + x;

    let version = &cfg.package.version;
    let mut definitions = vec![
        (name.clone(), "1".into()),
        (tag("_VERSION"), quote(&cfg.package.version)),
    ];

    let arr = ["_MAJOR", "_MINOR", "_PATCH"];
    for (i, item) in version.split('.').take(3).enumerate() {
        definitions.push((tag(arr[i]), item.into()))
    }
    definitions
}

fn append_defs(args: &mut Vec<String>, defs: &Vec<(String, String)>) {
    for (name, body) in defs {
        args.push(format!("-D{name}={body}"))
    }
}

fn main() -> Result<(), std::io::Error> {
    let contents = fs::read_to_string("chroma.toml")?;

    let config: Config = toml::from_str(&contents)?;

    let definitions = get_definitions(&config);
    println!("{:#?}", get_definitions(&config));

    println!("{:#?}", config);

    std::fs::create_dir_all("build").expect("Couldn't create build/ directory");
    for app in &config.bin {
        let mut args = vec![];
        append_defs(&mut args, &definitions);
        args.push("-o".into());
        args.push("build/".to_string() + app.name.as_str());
        args.push(format!("-std={}", config.package.edition));
        args.push(app.path.clone());
        if !Command::new("g++").args(args).spawn()?.wait()?.success() {
            eprintln!("Error when compiling {}", app.name);
            std::process::exit(1)
        }
    }

    Ok(())
}
