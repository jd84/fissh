use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use yaml_rust::{Yaml, YamlLoader};

pub fn parse_config_file<P: AsRef<Path>>(file: P) -> Result<(), Box<dyn Error>> {
    let mut file = File::open(&file)?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;

    let docs = YamlLoader::load_from_str(&data)?;
    let version = &docs[0]["version"]
        .as_i64()
        .ok_or("unknown config version")?;

    Ok(())
}

fn parse_identities(data: &Yaml) -> Result<(), Box<dyn Error>> {
    let identities = data["credentials"]
        .as_hash()
        .ok_or("error in credentials")?
        .iter()
        .map(|(n, _)| n)
        .collect::<Vec<_>>();

    Ok(())
}
