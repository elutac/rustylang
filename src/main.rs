use reqwest::blocking::Client;
use reqwest::header::HeaderMap;
use reqwest::StatusCode;
use dotenv::dotenv;
use serde_json::json;
use clap::{command, Arg};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a clap App to parse command-line arguments
    let matches = command!()
        .arg(
            Arg::new("tolang")
                .short('t')
                .long("tolang")
                .value_name("target language")
                .help("Sets the target language.")
                .required(false)
                .default_value("EN")
            )
        .arg(
            Arg::new("fromlang")
                .short('f')
                .long("fromlang")
                .value_name("source language")
                .help("Sets the source language.")
            )
        .arg(
            Arg::new("text")
            .help("Text to translate.")
            .value_name("text")
            .required(true)
            .index(1)
        )
        .get_matches();
    
    // Load environment variables
    dotenv().ok();

    // Create a reqwest Client
    let client = Client::new();

    // Define the API endpoint URL
    let url = "https://api-free.deepl.com/v2/translate";

    // Create an array of values for text
    let text = vec![matches.get_one::<String>("text").unwrap()];

    // Create a JSON object with the variables
    let data = json!({
        "text": text,
        "target_lang": matches.get_one::<String>("tolang").unwrap()
    });

    // Serialize the JSON object to a string
    let json_payload = serde_json::to_string(&data).unwrap();

    // Prepare the request body as a JSON payload (you can use other formats as needed)
    // let json_payload = r#"
    //     {
    //         "text" : ["This is a test."],
    //         "target_lang" : "Value1" 
    //     }
    // "#;

    // Define authentication token
    let auth_key = std::env::var("DEEPL_AUTH_KEY").expect("DEEPL_AUTH_KEY must be set.");

    // Custom headers
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse()?);
    headers.insert("Authorization", ("DeepL-Auth-Key ".to_owned() + &auth_key).parse()?);
    
    // Send the POST request
    let response = client
        .post(url)
        .headers(headers)
        .body(json_payload)
        .send()?;

    // Check the response status code
    match response.status() {
        StatusCode::OK => {
            // Request was successful (status code 200)
            println!("Request was successful!");
            // Deserialize the JSON response
            let json_response: serde_json::Value = response.json()?;
            // Print the JSON response
            println!("Response JSON: {:#?}", json_response);
        }
        _ => {
            // Request failed with an error status code
            println!("Request failed with status code: {:?}", response.status());
            // You can handle error responses here
        }
    }

    Ok(())
}
