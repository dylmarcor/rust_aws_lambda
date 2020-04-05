// My version of Julien Blanchard's email checker from
// http://julienblanchard.com/2015/rust-on-aws-lambda/

extern crate regex;
use regex::Regex;
use std::env;

fn main() {
    println!("Starting email-checker...");

    // Create an email regex
    let re = Regex::new(r"^\w+([-+.']\w+)*@\w+([-.]\w+)*\.\w+)*$").unwrap();

    // Match the first arg against the regex
    match env::args().nth(1) {
        // We have an argument
        Some(email) => {
            if re.is_match(&email) {
                println!("{} is a valid email.", email);
            } else {
                println!("{} is NOT a valid email.", email);
            }
        }

        // No argument provided
        None => {
            println!("Please provide a string to test.");
            return;
        }
    };
};
