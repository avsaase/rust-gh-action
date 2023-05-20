use std::{env, fs::write, process::exit};

use ureq::serde_json;

fn main() {
    let output_path = "output".to_string();

    let args: Vec<String> = env::args().collect();
    let url = &args[1];

    println!("Making GET request to {}", &url);

    match ureq::get(url).call() {
        Ok(response) => match response.into_json::<serde_json::Value>() {
            Ok(json) => write(output_path, format!("response={json}")).unwrap(),
            Err(error) => {
                eprint!("Error decoding response: {}", error);
                write(output_path, format!("error={error}")).unwrap();
                exit(1);
            }
        },
        Err(error) => {
            eprintln!("Error: {error}");
            write(output_path, format!("error={error}")).unwrap();
            exit(1);
        }
    }
}
