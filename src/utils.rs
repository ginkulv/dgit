use std::path::Path;
use crate::dbconnector::Entity;

pub fn repo_exists(dir_path: &str) -> bool {
    let path = format!("{}{}", dir_path, "/.dgit");
    Path::new(&path).exists()
}

pub fn strip_trailing_newline(input: &str) -> String {
    input
        .strip_suffix("\r\n")
        .or(input.strip_suffix("\n"))
        .unwrap_or(input)
        .to_string()
}

pub fn parse_argument(argument: &str) -> Result<Entity, String> {
    let result: Vec<&str> = argument.split(".").collect();
    if result.len() == 1 {
        return Err(format!("Invalid argument provided: {}", &argument))
    }
    let domain = result[0];
    let name = result[1];
    Ok(Entity::new(String::from(domain), String::from(name)))
}
