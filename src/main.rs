use serde::Deserialize;
use std::fs;
use std::process::Command;
use std::string::String;
use toml;

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

fn normalize(s: &str) -> String {
    let s = s.to_string().to_ascii_uppercase();
    return s.replace("-", "_");
}
fn quote(s: impl AsRef<str>) -> String {
    let s = s.as_ref();
    return "\"".to_string() + s + "\"";
}
fn get_definitions(cfg: &Config) -> Vec<(String, String)> {
    let name = normalize(cfg.package.name.as_str());

    let tag = |x| name.clone() + x;

    return vec![
        (tag(""), "1".into()),
        (tag("_VERSION"), quote(&cfg.package.version)),
    ];
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
