use std::{path::Path, io::{stdout, stdin, Write}};

pub fn repo_exists(dir_path: &str) -> bool {
    let path = format!("{}{}", dir_path, "/.dgit");
    Path::new(&path).exists()
}

pub fn strip_trailing_newline(input: &str) -> String {
    input
        .strip_suffix("\r\n")
        .or(input.strip_suffix("\n"))
        .unwrap_or(input)
        .to_owned()
}

pub fn read_string(print_str: &str) -> String {
    let mut string: String = String::new();
    print!("{}: ", print_str);
    stdout().flush().ok();
    stdin().read_line(&mut string).expect("Read string from stdin");
    strip_trailing_newline(&mut string)
}

pub fn parse_argument(argument: &str) -> Result<(&str, &str), String> {
    let result: Vec<&str> = argument.split(".").collect();
    if result.len() != 2 {
        return Err(format!("Invalid argument provided: {}", &argument))
    }
    let schema = result[0];
    let name = result[1];
    Ok((schema, name))
}
