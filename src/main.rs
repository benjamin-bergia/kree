use clap::{Parser, ValueEnum};
use serde::Deserialize;
use serde_json::json;
use serde_yaml;
use std::{
    fs,
    io,
    path::PathBuf,
};


#[derive(Parser)]
#[command()]
struct Args {
    /// Path to the kustomization file or directory
    path: PathBuf,

    /// Output format
    #[arg(short, long, value_enum, default_value = "text")]
    format: Option<Format>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Format {
    /// One path per line
    Text,

    /// JSON
    Json,
}


#[derive(Deserialize, PartialEq, Debug)]
struct Kustomization {
    #[serde(default)]
    resources: Vec<String>,
}


fn canonical_path(path: PathBuf) -> io::Result<PathBuf> {
    let mut canonical = path.canonicalize()?;

    if canonical.is_file() {
        return Ok(canonical);
    }

    if canonical.is_dir() {
        canonical.push("kustomization.yml");
        
        if canonical.is_file() {
            return Ok(canonical);
        }
    }

    let error = io::Error::new(
        io::ErrorKind::Other,
        format!("Invalid path {}", canonical.display())
    );
    return Err(error);
}


fn read_file(path: PathBuf) -> io::Result<String> {
    return fs::read_to_string(path);
}


fn deserialize(path: PathBuf) -> Vec<Kustomization> {
    let content = read_file(path).unwrap();
    
    return serde_yaml::Deserializer::from_str(&content)
        .map(|doc| Kustomization::deserialize(doc).unwrap())
        .collect();
}


fn run(path: PathBuf, result: &mut Vec<String>) {
    if let Ok(canonical) = canonical_path(path.clone()) {
        result.push(format!("{}", canonical.display()));

        let resources: Vec<String> = deserialize(canonical.clone())
            .iter()
            .map(|doc| doc.resources.clone())
            .flatten()
            .collect();

        for r in resources {
            let mut next_path = canonical
                .parent()
                .unwrap()
                .to_path_buf();
            next_path.push(PathBuf::from(r));

            run(next_path, result);
        };
    };
}


fn main() {
    let args = Args::parse();
    let mut result = Vec::new();

    run(args.path, &mut result);

    match args.format {
        Some(Format::Json) => {
            let json = json!(result);
            println!("{}", json.to_string());
        },
        _ => {
            for r in result.iter() {
                println!("{r}");
            }
        }
    }
}
