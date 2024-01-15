use std::{fs, io::Error};

use chrono::{DateTime, Utc, serde::ts_seconds};
use serde_json::{from_reader, to_string};
use crate::dbconnector::{Entity, Credentials};
use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Commit {
    pub entities: Vec<Entity>,
    #[serde(with = "ts_seconds")]
    pub timestamp: DateTime<Utc>,
}

pub fn read_credentials(dir_path: &str) -> Result<Credentials, Error> {
    let path = format!("{}{}", dir_path, "/.dgit/credentials");
    let file = fs::File::open(&path)?;
    let credentials: Credentials = from_reader(&file).unwrap();
    Ok(credentials)
}

pub fn store_credentials(dir_path: &str, credentials: Credentials) -> Result<(), Error> {
    let path = format!("{}{}", dir_path, "/.dgit/credentials");
    let json = to_string(&credentials).unwrap();
    std::fs::write(&path, &json)?;
    Ok(())
}

pub fn read_staged_entities(dir_path: &str) -> Result<Vec<Entity>, Error> {
    let path = format!("{}{}", dir_path, "/.dgit/stage");
    let file = fs::File::open(&path)?;
    let entities: Vec<Entity> = from_reader(&file).unwrap();
    Ok(entities)
}

pub fn store_staged_entities(dir_path: &str, entities_to_stage: Vec<Entity>) -> Result<(), Error> {
    let path = format!("{}{}", dir_path, "/.dgit/stage");
    let json = to_string(&entities_to_stage).unwrap();
    fs::write(&path, &json)?;
    Ok(())
}

pub fn read_commited_entities(dir_path: &str) -> Result<Vec<Commit>, Error> {
    let path = format!("{}{}", dir_path, "/.dgit/commit");
    let file = fs::File::open(&path)?;
    let commits: Vec<Commit> = from_reader(&file).unwrap();
    Ok(commits)
}

pub fn store_commited_entities(dir_path: &str, commits: Vec<Commit>) -> Result<(), Error> {
    let path = format!("{}{}", dir_path, "/.dgit/commit");
    let json = to_string(&commits).unwrap();
    fs::write(&path, &json)?;
    Ok(())
}
