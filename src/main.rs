use std::{env, io::stdout, time::Duration, thread, io::Write};
use reqwest::{self, Result};
use rand::{thread_rng, Rng};
use json;

// Get the fact data based on the URL that is passed
fn get_fact(url: &str) -> Result<String> {
    reqwest::blocking::get(url)
        .expect("Couldn't get fact, please check internet connection.")
        .text()
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
        thread::sleep(Duration::from_millis(100));
    }
    println!("");
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

    if args.len() > 1 { // If an argument is passed handle it
        match args[1].to_lowercase() {
            _ if "cat".to_string() == args[1] => type_string(&json::parse(&get_fact("https://catfact.ninja/fact").unwrap())
                                                    .unwrap()["fact"]
                                                    .to_string()),
            _ if "dog".to_string() == args[1] => type_string(&json::parse(&get_fact("http://dog-api.kinduff.com/api/facts?number=1").unwrap())
                                                    .unwrap()["facts"][0]
                                                    .to_string()),
            _ if "help".to_string() == args[1] => print_help(),
            _ => {
                println!("Invalid option: {}", args[1]);
                print_help();
            }
        };
    } else { // If no argument is passed choose a dog or a cat fact randomly
        if random_animal() == "cat".to_string() {
            type_string(&json::parse(&get_fact("https://catfact.ninja/fact").unwrap())
                .unwrap()["fact"]
                .to_string());
        } else {
            type_string(&json::parse(&get_fact("http://dog-api.kinduff.com/api/facts?number=1").unwrap())
                .unwrap()["facts"][0]
                .to_string());
        }
    }
}