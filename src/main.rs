mod dbconnector;
mod utils;
mod files;

use colored::Colorize;
use utils::*;
use files::*;
use dbconnector::{db_init, Entity, get_entities};
use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::io::{stdin, stdout, Write};
use std::path::Path;

fn init(dir_path: &str) {
    if repo_exists(dir_path) {
        println!("Repository already exists");
        std::process::exit(0);
    }
    let path = format!("{}{}", dir_path, "/.dgit");

    match fs::create_dir_all(&path) {
        Ok(_) => println!("Directory .dgit was created successfully"),
        Err(_) => { println!("Couldn't create directory {}!", &path); return }
    };

    let cred_path = format!("{}{}", path, "/.credentials");

    if Path::new(&cred_path).exists() {
        println!("File .credentials already exists");
        return
    }

    match fs::File::create(&cred_path) {
        Ok(_) => println!("File .credentials was created successfully"),
        Err(_) => { println!("Couldn't create the .credentials file"); return }
    };

    println!("Input your credentials for database:");

    let mut url: String = String::new();
    let mut dbname: String = String::new();
    let mut name: String = String::new();
    let mut password: String = String::new();

    print!("Url: ");
    stdout().flush().ok();
    stdin().read_line(&mut url).expect("Something went wrong");
    url = strip_trailing_newline(&mut url);

    print!("Database name: ");
    stdout().flush().ok();
    stdin().read_line(&mut dbname).expect("Something went wrong");
    dbname = strip_trailing_newline(&mut dbname);

    print!("Username: ");
    stdout().flush().ok();
    stdin().read_line(&mut name).expect("Something went wrong");
    name = strip_trailing_newline(&mut name);

    print!("Password: ");
    stdout().flush().ok();
    stdin().read_line(&mut password).expect("Something went wrong");
    password = strip_trailing_newline(&mut password);

    let mut credentials: BTreeMap<&str, &str> = BTreeMap::new();
    credentials.insert("url", &url);
    credentials.insert("dbname", &dbname);
    credentials.insert("name", &name);
    credentials.insert("password", &password);

    match store_credentials(dir_path, &credentials) {
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
            println!("File .credentials doesn't exists!");  // TODO suggest creating one
            return
        }
    };

    let name: &str = credentials.get("name").unwrap();
    let password: &str = credentials.get("password").unwrap();
    let url: &str = credentials.get("url").unwrap();
    let dbname: &str = credentials.get("dbname").unwrap();

    let mut client = match db_init(name, password, url, dbname) {
        Ok(client) => client,
        Err(_) => { println!("Coudln't connect to the database"); return; }
    };
    let entities: Vec<Entity> = get_entities(&mut client);
    let staged_entities: Vec<Entity> = read_staged(&dir_path).unwrap_or_default();

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

fn add(dir_path: &str, arguments: &[String]) {
    if !repo_exists(dir_path) {
        println!("Not in repository!");
        return
    }

    let credentials = match read_credentials(dir_path) {
        Ok(credentials) => credentials,
        Err(_) => { println!("File .credentials doesn't exists!"); return } // TODO suggest creating one
    };

    let name: &str = credentials.get("name").unwrap();
    let password: &str = credentials.get("password").unwrap();
    let url: &str = credentials.get("url").unwrap();
    let dbname: &str = credentials.get("dbname").unwrap();

    let mut client = match db_init(name, password, url, dbname) {
        Ok(client) => client,
        Err(_) => { println!("Coudln't connect to the database"); std::process::exit(0); }
    };
    let entities: Vec<Entity> = get_entities(&mut client);
    let mut entities_add: Vec<Entity> = Vec::new();

    for argument in arguments {
        let entity = match parse_argument(&argument) {
            Ok(entity) => entity,
            Err(error) => { println!("{}", error); return }
        };
        println!("{}", entity.to_string());
        entities_add.push(entity);
    }

    let mut tables: Vec<String> = Vec::new();
    for entity in entities_add {
        if entities.contains(&entity) {
            tables.push(entity.to_string());
        }
    }
    match store_staged(dir_path, &tables) {
        Ok(()) => println!("Added changes successfully"),
        Err(_) => { println!("Coudn't write the changes"); return }
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
        "add" => add(&current_dir, &args[2..]),
        _ => println!("Invalid command: {}", command)
    };
}
