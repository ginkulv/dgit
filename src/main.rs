mod dbconnector;
mod utils;
mod files;

use colored::Colorize;
use utils::*;
use files::*;
use dbconnector::{db_init, Entity, get_entities};
use std::env;
use std::fs;
use std::path::Path;

use crate::dbconnector::Credentials;

fn init(dir_path: &str) {
    if repo_exists(dir_path) {
        println!("Repository already exists");
        std::process::exit(0);
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
            return
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
            println!("File credentials doesn't exists!");  // TODO suggest creating one
            return
        }
    };

    let mut client = match db_init(&credentials) {
        Ok(client) => client,
        Err(_) => { println!("Coudln't connect to the database"); return; }
    };
    let entities: Vec<Entity> = get_entities(&mut client);
    let staged_entities: Vec<Entity> = read_staged_entities(&dir_path).unwrap_or_default();

    let mut untracked_entities: Vec<&Entity> = Vec::new();
    let mut entity_is_staged: bool;
    for entity in &entities {
        entity_is_staged = false;
        for staged in &staged_entities {
            if entity == staged {
                entity_is_staged = true;
                break;
            }
        }
        if !entity_is_staged {
            untracked_entities.push(entity);
        }
    }

    if untracked_entities.len() != 0 {
        println!("Untracked:");
        for entity in &untracked_entities {
            println!("    {}{}{}", entity.domain.red(), String::from(".").red(), entity.name.red());
        }
    }

    if staged_entities.len() != 0 {
        println!("Staged:");
        for staged in &staged_entities {
            println!("    {}{}{}", staged.domain.green(), String::from(".").green(), staged.name.green());
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
        Err(_) => { println!("File credentials doesn't exists!"); return } // TODO suggest creating one
    };

    let mut client = match db_init(&credentials) {
        Ok(client) => client,
        Err(_) => { println!("Coudln't connect to the database"); return; }
    };
    let entities: Vec<Entity> = get_entities(&mut client);
    let mut entities_to_stage: Vec<Entity> = Vec::new();

    for argument in arguments {
        let entity = match parse_argument(&argument) {
            Ok(entity) => entity,
            Err(error) => { println!("{}", error); return }
        };
        println!("{}", entity.to_string());
        if entities.contains(&entity) { 
            entities_to_stage.push(entity);
        }
    }

    let staged_entities: Vec<Entity> = read_staged_entities(dir_path).unwrap_or_default();
    for staged in staged_entities {
        if !entities_to_stage.contains(&staged) {
            entities_to_stage.push(staged);
        }
    }

    match store_staged(dir_path, entities_to_stage) {
        Ok(()) => println!("Staged successfully"),
        Err(_) => { println!("Coudln't stage"); return }
    };
}

fn unstage(dir_path: &str, arguments: &[String]) {
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
    let mut entities_to_unstage: Vec<Entity> = Vec::new();

    for argument in arguments {
        let entity = match parse_argument(&argument) {
            Ok(entity) => entity,
            Err(error) => { println!("{}", error); return }
        };
        println!("{}", entity.to_string());
        if entities.contains(&entity) { 
            entities_to_unstage.push(entity);
        }
    }

    let mut staged_entities: Vec<Entity> = read_staged_entities(dir_path).unwrap_or_default();
    staged_entities = staged_entities.into_iter().filter(|e| !entities_to_unstage.contains(e)).collect();

    match store_staged(dir_path, staged_entities) {
        Ok(()) => println!("Unstaged successfully"),
        Err(_) => { println!("Coudln't unstage"); return }
    };
}

fn main() {
    let current_dir: String = std::env::current_dir().unwrap().into_os_string().into_string().unwrap();
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("No arguments provided");
        std::process::exit(0);
    }

    let command: &str = &args[1];
    let _result = match command {
        "init" => init(&current_dir),
        "status" => status(&current_dir),
        "stage" => stage(&current_dir, &args[2..]),
        "unstage" => unstage(&current_dir, &args[2..]),
        _ => println!("Invalid command: {}", command)
    };
}
