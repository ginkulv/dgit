mod dbconnector;
mod utils;

use colored::Colorize;
use utils::*;
use dbconnector::{db_init, Entity, get_entities};
use serde_yaml::to_string;
use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::io::{stdin, stdout, Write};
use std::path::Path;

fn init(dir_path: &str) {
    if repo_exists(dir_path) {
        println!("Repository already exists");
        return
    }
    let path = format!("{}{}", dir_path, "/.dgit");

    match fs::create_dir_all(&path) {
        Ok(_res) => println!("Directory .dgit was created successfully"),
        Err(_err) => { println!("Couldn't create directory {}!", &path); return }
    };

    let cred_path = format!("{}{}", path, "/.credentials");

    if Path::new(&cred_path).exists() {
        println!("File .credentials already exists");
        return
    }

    match fs::File::create(&cred_path) {
        Ok(_res) => println!("File .credentials was created successfully"),
        Err(_err) => { println!("Couldn't create the .credentials file"); return }
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

    let yaml = to_string(&credentials).unwrap();
    match std::fs::write(&cred_path, &yaml) {
        Ok(_res) => println!("Repository was initialized successfully"),
        Err(_err) => {
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
        Err(_err) => {
            println!("File .credentials doesn't exists!");  // TODO suggest creating one
            return
        }
    };

    let name: &str = credentials.get("name").unwrap();
    let password: &str = credentials.get("password").unwrap();
    let url: &str = credentials.get("url").unwrap();
    let dbname: &str = credentials.get("dbname").unwrap();

    let mut client = db_init(name, password, url, dbname);
    let entities: Vec<Entity> = get_entities(&mut client);

    println!("Entitys found: {}.", entities.len());
    println!("Untracked tables:");
    for entity in entities {
        println!("    {}{}{}", entity.domain.green(), String::from(".").green(), entity.name.green());
    }
}

fn add(dir_path: &str, arguments: &[String]) {
    if !repo_exists(dir_path) {
        println!("Not in repository!");
        return
    }

    let credentials = match read_credentials(dir_path) {
        Ok(credentials) => credentials,
        Err(_err) => { println!("File .credentials doesn't exists!"); return } // TODO suggest creating one
    };

    let name: &str = credentials.get("name").unwrap();
    let password: &str = credentials.get("password").unwrap();
    let url: &str = credentials.get("url").unwrap();
    let dbname: &str = credentials.get("dbname").unwrap();

    let mut client = db_init(name, password, url, dbname);
    let entities: Vec<Entity> = get_entities(&mut client);
    let mut entities_add: Vec<Entity> = Vec::new();

    for argument in arguments {
        println!("Argument: {}", argument);
        let entity = match parse_argument(&argument) {
            Ok(entity) => entity,
            Err(error) => { println!("{}", error); return }
        };
        println!("{}", entity.to_string());
        entities_add.push(entity);
    }

    let mut changed: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut tables: Vec<String> = Vec::new();
    for entity in entities_add {
        if entities.contains(&entity) {
            tables.push(entity.to_string());
        }
    }
    changed.insert(String::from("tables"), tables);
    match store_added_changes(dir_path, &changed) {
        Ok(()) => println!("Added changes successfully"),
        Err(_err) => { println!("Coudn't write the changes"); return }
    };
}

fn commit(_dir_path: &str) {
}

fn push(_dir_path: &str) {
}

fn main() {
    let current_dir: String = std::env::current_dir().unwrap().into_os_string().into_string().unwrap();
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("No arguments provided");
    }

    let command: &str = &args[1];
    let _result = match command {
        "init" => init(&current_dir),
        "status" => status(&current_dir),
        "add" => add(&current_dir, &args[2..]),
        "commit" => commit(&current_dir),
        "push" => push(&current_dir),
        _ => panic!("Incorrect command: {} not found!", command)
    };
}
