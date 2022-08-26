mod dbconnector;

use dbconnector::{db_init, Table};
use std::env;
use std::io::{stdin, stdout, Write};
use std::fs;
use std::path::Path;

use crate::dbconnector::get_tables;


fn strip_trailing_newline(input: &str) -> String {
    input
        .strip_suffix("\r\n")
        .or(input.strip_suffix("\n"))
        .unwrap_or(input)
        .to_string()
}

fn init(dir_path: &str) {
	println!("{}{}", dir_path,  "/.dgit");
    fs::create_dir_all(format!("{}{}", dir_path, "/.dgit"))
        .unwrap_or_else(|err| { println!("Error occured:\n{}", err) });

    println!("Directory was created succesfully");

    println!("Input your credentials for database:");

	let cred_exists = Path::new(".credentials").exists();

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

    let conn_str = format!("postgresql://{}:{}@{}/{}", &name, &password, &url, &dbname);
    let mut client = db_init(&conn_str);
    let tables: Vec<Table> = get_tables(&mut client);

    println!("Tables found: {}.", tables.len());
    for table in tables {
        println!("    {}.{}", table.domain, table.name);
    }
}

fn status() {

}

fn main() {
	let current_dir: String = std::env::current_dir().unwrap().into_os_string().into_string().unwrap();
	println!("{}", current_dir);
    let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);

    if args.len() == 1 {
        println!("No arguments provided");
        return;
    }

    let command: &str = &args[1];

    match command {
        "init" => init(&current_dir),
        "status" => println!(&current_dir),
        "add" => println!("add"),
        "commit" => println!("commit"),
        "push" => println!("push"),
        _ => println!("Unknown command"),
    }
}
