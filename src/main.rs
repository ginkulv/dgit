mod dbconnector;

use colored::Colorize;
use dbconnector::{db_init, Table, get_tables};
use serde_yaml::to_string;
use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::io::{stdin, stdout, Write, Error};
use std::path::Path;

fn strip_trailing_newline(input: &str) -> String {
    input
        .strip_suffix("\r\n")
        .or(input.strip_suffix("\n"))
        .unwrap_or(input)
        .to_string()
}

fn init(dir_path: &str) -> Result<(), Error> {
    let path = format!("{}{}", dir_path, "/.dgit");

    if !Path::new(&path).exists() {
        fs::create_dir_all(&path)?;
        println!("Directory .dgit was created successfully");
    } else {
        println!("Direcotry .dgit already exists");
    }

    let cred_path = format!("{}{}", path, "/.credentials");

    if Path::new(&cred_path).exists() {
        println!("File .credentials already exists");
        return Ok(())
    } else {
        fs::File::create(&cred_path)?;
        println!("File .credentials was created successfully");
    }

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
    std::fs::write(&cred_path, &yaml)?;

    println!("Repository was initialized successfully");

    // let conn_str = format!("postgresql://{}:{}@{}/{}", &name, &password, &url, &dbname);
    // let mut client = db_init(&conn_str);
    // let tables: Vec<Table> = get_tables(&mut client);

    // println!("Tables found: {}.\n", tables.len());
    // for table in tables {
        // println!("    {}{}{}", table.domain.green(), String::from(".").green(), table.name.green());
    // }
    Ok(())
}

fn status(dir_path: &str) -> Result<(), Error> {
    Ok(())
}

fn add(dir_path: &str) -> Result<(), Error> {
    Ok(())
}

fn commit(dir_path: &str) -> Result<(), Error> {
    Ok(())
}

fn push(dir_path: &str) -> Result<(), Error> {
    Ok(())
}

fn main() -> Result<(), Error> {
    let current_dir: String = std::env::current_dir().unwrap().into_os_string().into_string().unwrap();
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("No arguments provided");
    }

    let command: &str = &args[1];

    match command {
        "init" => init(&current_dir),
        "status" => status(&current_dir),
        "add" => add(&current_dir),
        "commit" => commit(&current_dir),
        "push" => push(&current_dir),
        _ => panic!("Incorrect command: {} not found!", command)
    }
}
