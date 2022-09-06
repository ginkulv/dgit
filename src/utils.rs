#[path = "dbconnector.rs"]
mod dbconnector;

use std::fs;
use std::path::Path;
use std::collections::BTreeMap;
use std::io::Error;
use serde_yaml::from_reader;
use dbconnector::{Entity, EntityType};

pub fn repo_exists(dir_path: &str) -> bool {
    let path = format!("{}{}", dir_path, "/.dgit");
    Path::new(&path).exists()
}

pub fn read_credentials(dir_path: &str) -> Result<BTreeMap<String, String>, Error> {
    let path = format!("{}{}", dir_path, "/.dgit/.credentials");
    let file = fs::File::open(&path)?;
    let credentials: BTreeMap<String, String> = from_reader(&file).unwrap();
    Ok(credentials)
}

pub fn strip_trailing_newline(input: &str) -> String {
    input
        .strip_suffix("\r\n")
        .or(input.strip_suffix("\n"))
        .unwrap_or(input)
        .to_string()
}

pub fn parse_entity(argument: &str) -> Entity {
    let result: Vec<&str> = argument.split(".").collect();
    if result.len() == 1 {
        return
    }
    let domain = result[0];
    let name = result[1];
    Entity::new(String::from(domain), String::from(name), EntityType::NONE)
}
