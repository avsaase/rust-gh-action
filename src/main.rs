use std::{env, fs::write};

use ureq::serde_json;

fn main() {
    let github_output_path = env::var("GITHUB_OUTPUT").unwrap();
    println!("{}", &github_output_path);

    let args: Vec<String> = env::args().collect();
    let url = &args[1];

    println!("Making GET request to {}", &url);

    match ureq::get(url).call() {
        Ok(response) => match response.into_json::<serde_json::Value>() {
            Ok(json) => write(github_output_path, format!("response={json}")).unwrap(),
            Err(error) => {
                eprint!("Error decoding response: {}", error);
                write(github_output_path, format!("error={error}")).unwrap();
            }
        },
        Err(error) => {
            eprintln!("Error: {error}");
            write(github_output_path, format!("error={error}")).unwrap();
        }
    }
}
