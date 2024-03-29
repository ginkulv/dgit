mod dbconnector;
mod utils;
mod files;

use colored::Colorize;
use utils::*;
use files::*;
use dbconnector::{db_init, Entity, get_entities};
use uuid::Uuid;
use std::env;
use std::fs;
use std::path::Path;
use chrono::offset::Utc;

use crate::dbconnector::Credentials;

fn init(dir_path: &str) {
    if repo_exists(dir_path) {
        println!("Repository already exists");
        return;
    }
    let path = format!("{}{}", dir_path, "/.dgit");

    match fs::create_dir_all(&path) {
        Ok(_) => println!("Directory .dgit was created successfully"),
        Err(_) => { println!("Couldn't create directory {}", &path); return }
    };

    let cred_path = format!("{}{}", path, "/credentials");

    if Path::new(&cred_path).exists() {
        println!("File credentials already exists");
        return
    }

    match fs::File::create(&cred_path) {
        Ok(_) => println!("File credentials was created successfully"),
        Err(_) => { println!("Couldn't create the credentials file"); return }
    };

    println!("Input your credentials for database:");

    let url: String = read_string("Url");
    let dbname: String = read_string("Database name");
    let name: String = read_string("Username");
    let password: String = read_string("Password");

    let credentials: Credentials = Credentials {
        name,
        password,
        url,
        dbname,
    };

    match store_credentials(dir_path, credentials) {
        Ok(_) => println!("Repository was initialized successfully"),
        Err(_) => {
            print!("Coudn't save credentials");
        }
    };
}

fn status(dir_path: &str) {
    if !repo_exists(dir_path) {
        println!("Not in repository!");
        return
    }

    let credentials = match read_credentials(dir_path) {
        Ok(credentials) => credentials,
        Err(_) => {
            println!("File credentials doesn't exists!");
            return
        }
    };

    let mut client = match db_init(&credentials) {
        Ok(client) => client,
        Err(_) => { println!("Coudln't connect to the database"); return; }
    };

    let entities: Vec<Entity> = get_entities(&mut client);

    let staged_entities: Vec<Entity> = read_staged_entities(dir_path).unwrap_or_default();
    let commits: Vec<Commit> = read_commited_entities(dir_path).unwrap_or_default();
    let last_commit: Option<&Commit> = commits.last();
    let mut tracked_entities: &Vec<Entity> = &Vec::new();
    if let Some(commit) = last_commit {
        tracked_entities = &commit.entities;
    }

    let mut untracked_entities: Vec<&Entity> = Vec::new();
    for entity in &entities {
        if !staged_entities.contains(entity) && !tracked_entities.contains(entity) {
            untracked_entities.push(entity);
        }
    }

    let mut removed_entities: Vec<&Entity> = Vec::new();
    for entity in tracked_entities {
        if !entities.contains(entity) && !staged_entities.contains(entity) {
            removed_entities.push(entity);
        }
    }

    if removed_entities.len() != 0 {
        println!("Removed:");
        for removed in removed_entities {
            println!("    {}{}{}", removed.schema.magenta(), String::from(".").magenta(), removed.name.magenta());
        }
    }

    if tracked_entities.len() != 0 {
        println!("Tracked:");
        for tracked in tracked_entities {
            println!("    {}{}{}", tracked.schema.green(), String::from(".").green(), tracked.name.green());
        }
    }

    if untracked_entities.len() != 0 {
        println!("Untracked:");
        for entity in &untracked_entities {
            println!("    {}{}{}", entity.schema.red(), String::from(".").red(), entity.name.red());
        }
    }

    if staged_entities.len() != 0 {
        println!("Staged:");
        for staged in &staged_entities {
            println!("    {}{}{}", staged.schema.cyan(), String::from(".").cyan(), staged.name.cyan());
        }
    }
}

fn stage(dir_path: &str, arguments: &[String]) {
    if !repo_exists(dir_path) {
        println!("Not in repository!");
        return
    }

    let credentials = match read_credentials(dir_path) {
        Ok(credentials) => credentials,
        Err(_) => { println!("File credentials doesn't exists!"); return }
    };

    let mut client = match db_init(&credentials) {
        Ok(client) => client,
        Err(_) => { println!("Coudln't connect to the database"); return; }
    };
    let entities: Vec<Entity> = get_entities(&mut client);
    let mut entities_to_stage: Vec<Entity> = Vec::new();

    let mut staged_entities: Vec<Entity> = read_staged_entities(dir_path).unwrap_or_default();

    let commits: Vec<Commit> = read_commited_entities(dir_path).unwrap_or_default();
    let last_commit: Option<&Commit> = commits.last();
    let mut commited_entities: &Vec<Entity> = &Vec::new();
    if let Some(commit) = last_commit {
        commited_entities = &commit.entities;
    }

    for argument in arguments {
        let (schema, name) = match parse_argument(&argument) {
            Ok((schema, name)) => (schema, name),
            Err(error) => { println!("{}", error); return }
        };
        for entity in &entities {
            if entity.schema != schema || entity.name != name {
                continue;
            }

            let mut is_staged: bool = false;
            let mut is_changed: bool = false;

            for staged_entity in &mut staged_entities {
                if staged_entity.schema != schema || staged_entity.name != name {
                    continue;
                }

                if entity.columns != staged_entity.columns {
                    staged_entity.columns = entity.columns.clone();
                }
                is_staged = true;
            }

            for commited_entity in commited_entities {
                if commited_entity.schema != schema || commited_entity.name != name {
                    continue;
                }

                if entity.columns != commited_entity.columns {
                    entities_to_stage.push(entity.clone());
                }
                is_changed = true;
            }

            if !is_staged && !is_changed {
                entities_to_stage.push(entity.clone());
            }

        }
    }

    for staged in staged_entities {
        if !entities_to_stage.contains(&staged) {
            entities_to_stage.push(staged);
        }
    }

    match store_staged_entities(dir_path, entities_to_stage) {
        Ok(()) => {}
        Err(_) => println!("Coudln't stage")
    };
}

fn unstage(dir_path: &str, arguments: &[String]) {
    if !repo_exists(dir_path) {
        println!("Not in repository!");
        return
    }

    let mut staged_entities: Vec<Entity> = read_staged_entities(dir_path).unwrap_or_default();
    for argument in arguments {
        let (schema, name) = match parse_argument(&argument) {
            Ok((schema, name)) => (schema, name),
            Err(error) => { println!("{}", error); return }
        };
        staged_entities.retain(|e| e.schema != schema || e.name != name);
    }

    match store_staged_entities(dir_path, staged_entities) {
        Ok(()) => println!("Unstaged successfully"),
        Err(_) => println!("Coudln't unstage")
    };
}

fn commit(dir_path: &str) {
    if !repo_exists(dir_path) {
        println!("Not in repository!");
        return
    }

    let mut staged_entities = match read_staged_entities(dir_path) {
        Ok(staged) => staged,
        Err(_) => {
            println!("No staged changes");
            return
        }
    };

    if staged_entities.len() == 0 {
        println!("No staged entities found");
        return
    }

    let mut commits: Vec<Commit> = read_commited_entities(dir_path).unwrap_or_default();
    let last_commit: Option<&Commit> = commits.last();
    let mut tracked_entities: Vec<Entity> = Vec::new();
    if let Some(commit) = last_commit {
        tracked_entities = commit.entities.clone();
    }

    for tracked in tracked_entities {
        if !staged_entities.contains(&tracked) {
            staged_entities.push(tracked);
        }
    }

    let commit: Commit = Commit {
        entities: staged_entities.into_iter().filter(|e| e.exists).collect(),
        timestamp: Utc::now(),
        uuid: Uuid::new_v4().to_string()
    };

    commits.push(commit);

    match store_commited_entities(dir_path, commits) {
        Ok(()) => println!("Changes commited successfully"),
        Err(_) => {
            println!("");
            return;
        }
    };

    match store_staged_entities(dir_path, Vec::new()) {
        Ok(()) => (),
        Err(_) => {
            println!("Coudln't clear staged");
        }
    };
}

fn remove(dir_path: &str, arguments: &[String]) {
    if !repo_exists(dir_path) {
        println!("Not in repository!");
        return
    }

    let credentials = match read_credentials(dir_path) {
        Ok(credentials) => credentials,
        Err(_) => { println!("File credentials doesn't exists!"); return }
    };

    let mut client = match db_init(&credentials) {
        Ok(client) => client,
        Err(_) => { println!("Coudln't connect to the database"); return; }
    };
    let entities: Vec<Entity> = get_entities(&mut client);
    let mut entities_to_remove: Vec<Entity> = Vec::new();

    let commits: Vec<Commit> = read_commited_entities(dir_path).unwrap_or_default();
    let last_commit: Option<&Commit> = commits.last();
    let mut tracked_entities: &Vec<Entity> = &Vec::new();
    if let Some(commit) = last_commit {
        tracked_entities = &commit.entities;
    }

    for argument in arguments {
        let (schema, name) = match parse_argument(&argument) {
            Ok((schema, name)) => (schema, name),
            Err(error) => { println!("{}", error); return }
        };

        let mut is_tracked: bool = false;
        for entity in tracked_entities {
            if entity.schema == schema && entity.name == name {
                is_tracked = true;
            }
        }

        if !is_tracked {
            println!("{}.{} is not tracked", schema, name);
            continue;
        }

        for entity in &entities {
            if entity.schema == schema && entity.name == name {
                let mut entity_to_remove = entity.clone();
                entity_to_remove.exists = false;
                entities_to_remove.push(entity_to_remove);
            }
        }
    }

    let staged_entities: Vec<Entity> = read_staged_entities(dir_path).unwrap_or_default();
    for staged in staged_entities {
        if !entities_to_remove.contains(&staged) {
            entities_to_remove.push(staged);
        }
    }

    match store_staged_entities(dir_path, entities_to_remove) {
        Ok(()) => {},
        Err(_) => { println!("Coudln't stage"); }
    };
}

fn log(dir_path: &str) {
    if !repo_exists(dir_path) {
        println!("Not in repository!");
        return
    }
    let commits: Vec<Commit> = read_commited_entities(dir_path).unwrap_or_default();
    for commit in commits {
        println!("Commit: {}", commit.uuid);
        println!("Timestamp: {}", commit.timestamp);
        for entity in commit.entities {
            println!("Tracked: {}.{}", entity.schema, entity.name);
        }
        println!("");
    }
}

fn main() {
    let current_dir: String = std::env::current_dir().unwrap().into_os_string().into_string().unwrap();
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("No arguments provided");
        return;
    }

    let command: &str = &args[1];
    let _result = match command {
        "init" => init(&current_dir),
        "status" => status(&current_dir),
        "stage" => stage(&current_dir, &args[2..]),
        "unstage" => unstage(&current_dir, &args[2..]),
        "commit" => commit(&current_dir),
        "remove" => remove(&current_dir, &args[2..]),
        "log" => log(&current_dir),
        _ => println!("Invalid command: {}", command)
    };
}
