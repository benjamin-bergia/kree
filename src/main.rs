use clap::{Parser, ValueEnum};
use relative_path::RelativePathBuf;
use serde::Deserialize;
use serde_json::json;
use serde_yaml;
use std::{
    env::current_dir,
    fs,
    io,
    path::PathBuf,
};


#[derive(Parser)]
#[command()]
struct Args {
    /// Path to the kustomization file or directory
    path: RelativePathBuf,

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


fn normalize(root: &PathBuf, mut path: RelativePathBuf) -> RelativePathBuf {
    path.normalize();

    if path.to_logical_path(&root).is_dir() {
        path.push("kustomization.yml");
    }

    if path.to_logical_path(&root).is_file() {
        return path;
    } else {
        path.set_extension("yaml");
    }

    if path.to_logical_path(&root).is_file() {
        return path;
    }

    panic!("Unable to normalize path {path}");
}


fn read_file(path: &PathBuf) -> io::Result<String> {
    return fs::read_to_string(path);
}


fn deserialize(path: &PathBuf) -> Vec<Kustomization> {
    let content = read_file(path).unwrap();
    
    return serde_yaml::Deserializer::from_str(&content)
        .map(|doc| Kustomization::deserialize(doc).unwrap())
        .collect();
}


fn run(root: &PathBuf, path: RelativePathBuf, result: &mut Vec<String>) {
    let current_path = normalize(&root, path.clone());

    result.push(format!("{}", current_path));

    let resources: Vec<String> = deserialize(&current_path.to_logical_path(root))
        .iter()
        .map(|doc| doc.resources.clone())
        .flatten()
        .collect();

    for r in resources {
        let next_path = current_path
            .parent()
            .unwrap()
            .join_normalized(r);

        run(root, next_path, result);
    };
}


fn main() {
    let args = Args::parse();
    let root = current_dir().unwrap();
    let mut result = Vec::new();

    run(&root, args.path, &mut result);

    result.sort();
    result.dedup();

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
