use clap::Parser;
use serde::Deserialize;
use serde_yaml;
use std::{
    fs,
    io,
    path::PathBuf,
};


#[derive(Parser)]
#[command()]
struct Args {
    path: PathBuf,
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


fn run(path: PathBuf, result: Vec<PathBuf>) {
    let canonical = canonical_path(path.clone()).unwrap();
    println!("{}", canonical.display());

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

        let mut branch = result.clone();

        branch.push(canonical.clone());
        run(next_path, branch);
    };
}


fn main() {
    let args = Args::parse();

    run(args.path, Vec::new());
}
