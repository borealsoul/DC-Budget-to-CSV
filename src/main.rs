use regex::Regex;
use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;

fn read_file_from_arg(file_path: &String) -> String {
    fs::read_to_string(file_path).expect("Should have been able to read the file")
}

fn write_file_from_arg(file_path: &String, text: String) {
    let mut output = File::create(file_path).unwrap();
    write!(output, "{}", text).unwrap();
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let transcript = read_file_from_arg(&args[1]);

    let rg_server_apps = Regex::new(r"(?m)Server\nAPP\n— ([\s\S]*?)\n{2}").unwrap();
    let server_apps_vec = rg_server_apps
        .captures_iter(transcript.as_str())
        .filter_map(|cap| cap.get(1).map(|m| m.as_str()))
        .collect::<Vec<_>>();
    let server_apps = server_apps_vec.join("\n");

    let rg_date_newlines = Regex::new(r"(AM|PM|])(\n)").unwrap();
    let date_newlines = rg_date_newlines
        .replace_all(server_apps.as_str(), |caps: &regex::Captures| {
            format!("{} ", &caps[1])
        })
        .to_string();

    let rg_add_semicolon = Regex::new(
        r"(/\d{4}|AM|PM)( +|] +)|( » )|(\n\[\d+:)|( has been unfined )|(\d) by |( for | for: )",
    )
    .unwrap();
    let add_semicolon = rg_add_semicolon
        .replace_all(date_newlines.as_str(), |caps: &regex::Captures| {
            if let Some(m) = caps.get(1).or_else(|| caps.get(6)) {
                format!("{};", m.as_str())
            } else if let Some(m) = caps.get(4) {
                format!("\n;{}", &m.as_str()[2..])
            } else {
                ";".to_string()
            }
        })
        .to_string();

    let rg_remove_dates = Regex::new(r"\w+ \d*, \d+\n").unwrap();
    let remove_dates = rg_remove_dates.replace_all(add_semicolon.as_str(), "");

    println!("{}", &remove_dates);

    write_file_from_arg(&args[2], remove_dates.to_string());
}
