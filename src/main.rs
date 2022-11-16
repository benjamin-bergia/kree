use clap::{Parser, ValueEnum};
use serde::Deserialize;
use serde_json::json;
use serde_yaml;
use std::{
    env::current_dir,
    fs,
    io,
    path::PathBuf,
};
use log::debug;


#[derive(Parser)]
#[command()]
struct Args {
    /// Path(s) to the kustomization file or directory
    path: Vec<PathBuf>,

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


#[derive(Default, Deserialize, PartialEq, Debug)]
#[serde(default, rename_all = "camelCase")]
struct Kustomization {
    resources: Vec<String>,
    config_map_generator: Vec<ConfigMapGenerator>,
}


#[derive(Clone, Default, Deserialize, PartialEq, Debug)]
#[serde(default)]
struct ConfigMapGenerator {
    name: String,
    files: Vec<String>,
}


fn normalize(path: PathBuf) -> PathBuf {
    let mut canonical = path.canonicalize().unwrap();

    if canonical.is_dir() {
        canonical.push("kustomization.yml");
    }

    if canonical.is_file() {
        return canonical;
    } else {
        canonical.set_extension("yaml");
    }

    if canonical.is_file() {
        return canonical;
    }

    panic!("Unable to normalize path {}", path.display());
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


fn unsupported(resource: &str) -> bool {
    let remote = [
        "git://",
        "http://",
        "https://",
        "github.com",
    ];

    for r in remote {
        if resource.starts_with(r) {
            return true;
        }
    }

    return false;
}

fn run(path: PathBuf, result: &mut Vec<String>) {
    debug!("{:#?}", path.display());

    let current_path = normalize(path.clone());
    debug!("{:#?}", current_path.display());

    result.push(format!("{}", current_path.display()));

    let doc = deserialize(&current_path);
    debug!("{:#?}", doc);

    doc
        .iter()
        .map(|doc| doc.config_map_generator.clone())
        .flatten()
        .map(|c| c.files)
        .flatten()
        .for_each(|f| {
            let file = current_path
                .parent()
                .unwrap()
                .join(f);

            result.push(format!("{}", file.display()));
        });


    let resources: Vec<String> = doc
        .iter()
        .map(|doc| doc.resources.clone())
        .flatten()
        .collect();

    for r in resources {
        if unsupported(&r) {
            continue;
        };

        let next_path = current_path
            .parent()
            .unwrap()
            .join(r);

        run(next_path, result);
    };
}


fn main() {
    env_logger::init();

    let args = Args::parse();
    let root = current_dir().unwrap();
    let mut result = Vec::new();

    for p in args.path.iter() {
        // Merge the current path with the input path.
        // If the input path is absolute it will overwrite
        // the current path.
        let mut path = root.clone();
        path.push(p);
        run(path, &mut result);
    }

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
