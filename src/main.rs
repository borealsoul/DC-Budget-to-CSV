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

fn transcript_to_csv(transcript: String) -> String {
    // ## REGEX EXPLAINATION ##
    // Captures every character between
    // "Server
    //  APP"
    // until it finds a double line break.
    let server_app_text = Regex::new(r"(?m)Server\nAPP\n— ([\s\S]*?)\n{2}")
        .unwrap()
        .captures_iter(transcript.as_str())
        .filter_map(|cap| cap.get(1).map(|m| m.as_str())) // Maps every capture to a &str
        .collect::<Vec<_>>() // Collects it to a vector
        .join("\n"); // Joins every item into a String, divided by line breaks

    // ## REGEX EXPLAINATION ##
    // Captures linebreaks after AM/PM/] and replaces it with a space
    let breaklines_to_space = Regex::new(r"(AM|PM|])(\n)")
        .unwrap()
        .replace_all(server_app_text.as_str(), |caps: &regex::Captures| {
            format!("{} ", &caps[1])
        })
        .to_string();

    // ## REGEX EXPLAINATION ##
    // Captures:
    // 1. Any space or "](space)" after a 4-digit number, AM or PM -- 2025]
    // 2. (space)»(space)
    // 3. (linebreak)[(digits): -- \n[4:
    // 4. "(space)has been unfined(space)"
    // 5. (digit)(space)by(space) -- 0 by
    // 6. (space)for(space) or (space)for:(space)
    let divide_columns_with_semicolons = Regex::new(
        r"(/\d{4}|AM|PM)( +|] +)|( » )|(\n\[\d+:)|( has been unfined )|(\d) by |( for | for: )",
    )
    .unwrap()
    .replace_all(breaklines_to_space.as_str(), |caps: &regex::Captures| {
        // WARNING: the number groups won't match
        // if it's the 1st or 5th capture group
        if let Some(m) = caps.get(1).or_else(|| caps.get(6)) {
            format!("{};", m.as_str())
        // if it's the 3rd capture group
        } else if let Some(m) = caps.get(4) {
            format!("\n;{}", &m.as_str()[2..])
        // whatever else
        } else {
            ";".to_string()
        }
    })
    .to_string();

    // ## REGEX EXPLAINATION ##
    // Captures a date with the format MMM dd, yyyy(linebreak)
    let remove_dates = Regex::new(r"\w+ \d*, \d+\n")
        .unwrap()
        .replace_all(divide_columns_with_semicolons.as_str(), "");

    remove_dates.to_string()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let csv_file = transcript_to_csv(read_file_from_arg(&args[1].to_string()));

    write_file_from_arg(&args[2], csv_file.to_string());
}
