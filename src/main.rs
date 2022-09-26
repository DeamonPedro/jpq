use clap::Parser;
use jsonpath_rust::JsonPathQuery;
use serde_json::Value;
use std::{fs, io};

#[derive(Parser, Debug)]
#[clap(author, version, about="Simple JSONPath tool", long_about = None)]
struct Args {
    /// Input JSON file
    #[clap(short, long, value_parser)]
    file: Option<String>,
    /// JSONPath selector
    #[clap(forbid_empty_values = true)]
    query: String,
    /// Output in plain text
    #[clap(short, long)]
    pain_text: bool,
}

fn error_message(msg: &str) -> ! {
    eprintln!("[Error] {}", msg);
    std::process::exit(1);
}

fn main() {
    let args = Args::parse();
    let mut buffer = String::new();
    if args.file.is_none() {
        for line in io::stdin().lines() {
            buffer.push_str(&line.unwrap());
        }
    } else {
        let file_path = args.file.unwrap();
        buffer = match fs::read_to_string(&file_path) {
            Ok(content) => content,
            _ => error_message(format!("Failed to read file \"{}\"", &file_path).as_str()),
        };
    }
    let json: Value = match serde_json::from_str(&buffer) {
        Ok(v) => v,
        Err(e) => match e.classify() {
            serde_json::error::Category::Io => error_message("Failed to read IO Stream"),
            serde_json::error::Category::Syntax => {
                error_message(format!("Invalid JSON: {}", e).as_str())
            }
            serde_json::error::Category::Data => {
                error_message(format!("Invalid JSON: {}", e).as_str())
            }
            serde_json::error::Category::Eof => {
                error_message(format!("Invalid JSON: {}", e).as_str())
            }
        },
    };
    let finder = match json.path(&args.query) {
        Ok(res) => res.as_array().unwrap().to_vec(),
        Err(e) => error_message(format!("Invalid JSONPath:\n{}", e).as_str()),
    };
    for item in &finder {
        match args.pain_text {
            true => println!("{}", item),
            false => println!("{}", serde_json::to_string_pretty(item).unwrap()),
        }
    }
}
