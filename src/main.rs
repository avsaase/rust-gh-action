use std::env;

use ureq::serde_json;

fn main() {
    // let github_output_path = env::var("GITHUB_OUTPUT").unwrap();

    let args: Vec<String> = env::args().collect();
    let input = &args[1];

    println!("{}", input);

    let response: serde_json::Value = ureq::get("https://httpbin.org/get")
        .call()
        .unwrap()
        .into_json()
        .unwrap();

    println!("{}", response["headers"]["Referer"]);
}
