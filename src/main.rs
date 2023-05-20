use std::{env, error::Error, fs::write, process::exit};

use ureq::serde_json;

const OUTPUT_PATH: &str = "output";

fn main() {
    let args: Vec<String> = env::args().collect();
    let url = &args[1];

    println!("Making GET request to {}", &url);
    match ureq::get(url).call() {
        Ok(response) => match response.into_json::<serde_json::Value>() {
            Ok(json) => write(OUTPUT_PATH, format!("response={json}")).unwrap(),
            Err(error) => return_error(error),
        },
        Err(error) => return_error(error),
    }
}

fn return_error(error: impl Error) {
    eprintln!("Error: {}", error);
    write(OUTPUT_PATH, format!("error={}", error)).unwrap();
    exit(1);
}
