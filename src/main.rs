use json;
use rand::{thread_rng, Rng};
use reqwest::{self, Result};
use std::{env, io::{stdout}, io::Write, thread, time::Duration, fs};
use toml::Table;

// Define custom error messages
fn print_error_message(animal: &str) -> String {
    let cat_error_message = String::from("Failed to retrieve Cat Fact, check internet connection.");
    let dog_error_message = String::from("Failed to retrieve Dog Fact, check internet connection.");

    if animal == "cat" {
        cat_error_message
    } else {
        dog_error_message
    }
}

// Get petfacts version from Cargo.toml
fn get_version() -> String {
    let toml_file = fs::read_to_string("Cargo.toml")
        .expect("Couldn't find Cargo.toml, this is a major problem")
        .parse::<Table>()
        .unwrap();
    toml_file["package"]["version"].to_string()
}

// Get the fact data based on the URL that is passed
fn get_fact(url: &str) -> Result<String> {
    match reqwest::blocking::get(url) {
        Ok(fact) => return fact.text(),
        Err(e) => return Err(e),
    };
}

// Return dog or cat as a string randomly
fn random_animal() -> String {
    if thread_rng().gen() {
        String::from("dog")
    } else {
        String::from("cat")
    }
}

// Give the output an effect of being typed out
fn type_string(string_to_type: &String) {
    for character in string_to_type.chars() {
        print!("{character}");
        stdout().flush().expect("Something is wrong with stdout!");
        thread::sleep(Duration::from_millis(50));
    }
    println!(""); // Go to the next line when done "typing"
}

// Print out a help screen
fn print_help() {
    println!("-----------------------------------------------------------");
    println!("                      -- Petfacts --                       ");
    println!("[dog] -- Print out a random dog fact to the console screen.");
    println!("[cat] -- Print out a random cat fact to the console screen.");
    println!("[help] -- Print this help message.                         ");
    println!("-----------------------------------------------------------");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        // If an argument is passed handle it
        match args[1].to_lowercase() {
            _ if "cat".to_string() == args[1] => {
                let fact = get_fact("https://catfact.ninja/fact");
                match fact {
                    Ok(f) => {
                        let fact_json = json::parse(&f.as_str()).unwrap();
                        type_string(&fact_json["fact"].to_string());
                    }

                    Err(_) => println!("{}", print_error_message("cat")),
                };
            }
            _ if "dog".to_string() == args[1] => {
                let fact = get_fact("http://dog-api.kinduff.com/api/facts?number=1");
                match fact {
                    Ok(f) => {
                        let fact_json = json::parse(&f.to_string()).unwrap();
                        type_string(&fact_json["facts"][0].to_string());
                    }

                    Err(_) => println!("{}", print_error_message("dog")),
                };
            }
            _ if "help".to_string() == args[1] => print_help(),
            _ if "version".to_string() == args[1] => println!("Petfacts Version: {}", get_version()),
            _ => {
                println!("Invalid option: {}", args[1]);
                print_help();
            }
        };
    } else {
        // If no argument is passed choose a dog or a cat fact randomly
        if random_animal() == "cat".to_string() {
            let fact = get_fact("https://catfact.ninja/fact");
            match fact {
                Ok(f) => {
                    let fact_json = json::parse(&f.as_str()).unwrap();
                    type_string(&fact_json["fact"].to_string());
                }

                Err(_) => println!("{}", print_error_message("cat")),
            };
        } else {
            let fact = get_fact("http://dog-api.kinduff.com/api/facts?number=1");
            match fact {
                Ok(f) => {
                    let fact_json = json::parse(&f.to_string()).unwrap();
                    type_string(&fact_json["facts"][0].to_string());
                }

                Err(_) => println!("{}", print_error_message("dog")),
            };
        }
    }
}
