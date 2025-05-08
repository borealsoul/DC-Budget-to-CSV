use fancy_regex::Regex;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    println!("In file {file_path}");

    let transcript = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let rg_server_apps = Regex::new("(?<=Server\nAPP\n)(\n|.)*?(?=\n\n)").unwrap();
    let server_apps = rg_server_apps
        .captures(transcript.as_str())
        .expect("Error running regex")
        .expect("Match not found");

    println!("{:?}", &server_apps[1]);
}
