use clap::{Arg, ArgMatches, Command};
use reqwest::{self, Result, StatusCode};
use std::{collections::HashMap, fs::OpenOptions, io::Write};
use tokio;

fn parse_arguments() -> ArgMatches {
    Command::new("anime quoter")
        .about("a simple app that générate anime quote throught an API")
        .version("0.0.1 bêta")
        .author("akito-sama")
        .arg(
            Arg::new("character")
                .help("the character that says the quote")
                .short('c')
                .long("character")
                .takes_value(true),
        )
        .arg(
            Arg::new("anime")
                .help("the anime that quote is from")
                .short('a')
                .long("anime")
                .takes_value(true),
        )
        .arg(
            Arg::new("file")
                .help("the output file that you want to stock in the quote")
                .short('f')
                .long("file")
                .takes_value(true),
        )
        .get_matches()
}

async fn get_data() -> Result<HashMap<String, String>> {
    let matches = parse_arguments();
    let url = "https://animechan.vercel.app/api/random/";
    let mut final_url = url.to_owned();
    if let Some(anime) = matches.value_of("anime") {
        final_url = format!("{}anime?title={}&", final_url, anime)
    }
    if let Some(character) = matches.value_of("character") {
        final_url = format!("{}character?name={}", url, character);
    }
    let data: HashMap<String, String> = match reqwest::get(final_url).await {
        Ok(reponse) => match reponse.status() {
            StatusCode::OK => match reponse.json().await {
                Ok(text) => text,
                Err(_) => panic!("can't parse JSON"),
            },
            StatusCode::NOT_FOUND => panic!("there is no quote with the argument you given"),
            _ => panic!("status is différent than 200"),
        },
        Err(_) => panic!("can't do the reqwest maybe cause of connection"),
    };
    if let Some(f) = matches.value_of("file") {
        match OpenOptions::new().append(true).write(true).open(f) {
            Ok(mut file) => {
                match file.write_all(
                    format!(
                         "{}\n\t\t\t\t --- {}\n\t\t\t\t --- {}\n\n",
                        data["quote"], data["character"], data["anime"]
                    )
                    .as_bytes(),
                ) {
                    Ok(_) => {},
                    Err(e) => println!("there is an error {}", e)
                };
            }
            Err(e) => {
                println!("can't open the file {}", e);
            }
        }
    }
    Ok(data)
}

#[tokio::main]
async fn main() {
    let data = get_data().await.unwrap();
    println!(
        "{}\n\t\t\t\t --- {}\n\t\t\t\t --- {}",
        data["quote"], data["character"], data["anime"]
    );
}
