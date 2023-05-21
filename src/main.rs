use std::{env, error::Error, fmt::Display, fs::write};

use ureq::serde_json;

const OUTPUT_PATH: &str = "output";

fn main() {
    let args: Vec<String> = env::args().collect();
    let url = &args[1];

    println!("Making GET request to {}", &url);
    let response = ureq::get(url).call().map_err(return_error).unwrap();
    let json = response
        .into_json::<serde_json::Value>()
        .map_err(return_error)
        .unwrap();
    return_output("response", json);
}

fn return_output<T: Display>(key: &str, value: T) {
    println!("{}:{}", key, value);
    write(OUTPUT_PATH, format!("{}={}", key, value)).unwrap();
}

fn return_error<E: Error>(error: E) {
    return_output("error", error);
    // exit(1);
}
