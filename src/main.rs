use reqwest::blocking::Client;
use reqwest::header::HeaderMap;
use reqwest::StatusCode;
use dotenv::dotenv;
use serde_json::json;
use clap::{command, Arg, builder::PossibleValue};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a clap App to parse command-line arguments
    let matches = command!()
        .arg(
            Arg::new("tolang")
                .short('t')
                .long("tolang")
                .value_name("target language")
                .help("Sets the target language.")
                .required(true)
                //.default_value("EN")
                .value_parser([
                    PossibleValue::new("BG").help("Bulgarian"),
                    PossibleValue::new("CS").help("Czech"),
                    PossibleValue::new("DA").help("Danish"),
                    PossibleValue::new("DE").help("German"),
                    PossibleValue::new("EL").help("Greek"),
                    PossibleValue::new("EN").help("English (unspecified variant for backward compatibility; please select EN-GB or EN-US instead)"),
                    PossibleValue::new("EN-GB").help("English (British)"),
                    PossibleValue::new("EN-US").help("English (American)"),
                    PossibleValue::new("ES").help("Spanish"),
                    PossibleValue::new("ET").help("Estonian"),
                    PossibleValue::new("FI").help("Finnish"),
                    PossibleValue::new("FR").help("French"),
                    PossibleValue::new("HU").help("Hungarian"),
                    PossibleValue::new("ID").help("Indonesian"),
                    PossibleValue::new("IT").help("Italian"),
                    PossibleValue::new("JA").help("Japanese"),
                    PossibleValue::new("KO").help("Korean"),
                    PossibleValue::new("LT").help("Lithuanian"),
                    PossibleValue::new("LV").help("Latvian"),
                    PossibleValue::new("NB").help("Norwegian (Bokmål)"),
                    PossibleValue::new("NL").help("Dutch"),
                    PossibleValue::new("PL").help("Polish"),
                    PossibleValue::new("PT").help("Portuguese (unspecified variant for backward compatibility; please select PT-BR or PT-PT instead)"),
                    PossibleValue::new("PT-BR").help("Portuguese (Brazilian)"),
                    PossibleValue::new("PT-PT").help("Portuguese (all Portuguese varieties excluding Brazilian Portuguese)"),
                    PossibleValue::new("RO").help("Romanian"),
                    PossibleValue::new("RU").help("Russian"),
                    PossibleValue::new("SK").help("Slovak"),
                    PossibleValue::new("SL").help("Slovenian"),
                    PossibleValue::new("SV").help("Swedish"),
                    PossibleValue::new("TR").help("Turkish"),
                    PossibleValue::new("UK").help("Ukrainian"),
                    PossibleValue::new("ZH").help("Chinese (simplified)")
                ])
            )
        .arg(
            Arg::new("fromlang")
                .short('f')
                .long("fromlang")
                .value_name("source language")
                .help("Sets the source language.")
                .value_parser([
                    PossibleValue::new("BG").help("Bulgarian"),
                    PossibleValue::new("CS").help("Czech"),
                    PossibleValue::new("DA").help("Danish"),
                    PossibleValue::new("DE").help("German"),
                    PossibleValue::new("EL").help("Greek"),
                    PossibleValue::new("EN").help("English"),
                    PossibleValue::new("ES").help("Spanish"),
                    PossibleValue::new("ET").help("Estonian"),
                    PossibleValue::new("FI").help("Finnish"),
                    PossibleValue::new("FR").help("French"),
                    PossibleValue::new("HU").help("Hungarian"),
                    PossibleValue::new("ID").help("Indonesian"),
                    PossibleValue::new("IT").help("Italian"),
                    PossibleValue::new("JA").help("Japanese"),
                    PossibleValue::new("KO").help("Korean"),
                    PossibleValue::new("LT").help("Lithuanian"),
                    PossibleValue::new("LV").help("Latvian"),
                    PossibleValue::new("NB").help("Norwegian (Bokmål)"),
                    PossibleValue::new("NL").help("Dutch"),
                    PossibleValue::new("PL").help("Polish"),
                    PossibleValue::new("PT").help("Portuguese (all Portuguese varieties mixed)"),
                    PossibleValue::new("RO").help("Romanian"),
                    PossibleValue::new("RU").help("Russian"),
                    PossibleValue::new("SK").help("Slovak"),
                    PossibleValue::new("SL").help("Slovenian"),
                    PossibleValue::new("SV").help("Swedish"),
                    PossibleValue::new("TR").help("Turkish"),
                    PossibleValue::new("UK").help("Ukrainian"),
                    PossibleValue::new("ZH").help("Chinese"),
                ])
            )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("Verbose output.")
                .required(false)
                .num_args(0)
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

    // Variable
    let mut detected_source_language: Option<String> = None;

    // Print response if successful
    match response.status() {
        StatusCode::OK => {
            // Request was successful (status code 200)
            // Deserialize the JSON response
            match response.json::<serde_json::Value>() {
                Ok(json_response) => {
                    // Extract the "text" field from the JSON response
                    if let Some(translations) = json_response["translations"].as_array() {
                        if let Some(translation) = translations.get(0) {
                            if let Some(text) = translation["text"].as_str() {
                                println!("{}", text);
                            }
                            if let Some(detected_lang) = translation["detected_source_language"].as_str() {
                                detected_source_language = Some(detected_lang.to_string());
                            }
                        }
                    }
            
                    if matches.get_flag("verbose") {
                        println!("");
                        println!("Input: {}", matches.get_one::<String>("text").unwrap());
                        println!("Detected Language: {}", detected_source_language.unwrap_or_else(|| "N/A".to_string()));
                        println!("Target Language: {}", matches.get_one::<String>("tolang").unwrap());
                    };
                },
                Err(err) => {
                    eprintln!("Failed to deserialize JSON response: {}", err);
                }
            }
        },
        status => {
            // Request failed with an error status code
            eprintln!("Request failed with status: {}", status);
            match response.text() {
                Ok(text) => eprintln!("Response text: {}", text),
                Err(err) => eprintln!("Failed to read response text: {}", err),
            }
        }
    }


    Ok(())
}
