use std::io;
use core::ops::Add;
use regex::Regex;

static WHITESPACE : Regex = regex!(r"^\s*$");

pub fn ask_with_default (message:&str, default:String) -> String {
    ask_with_custom_default(message, default.as_str(), &default)
}

pub fn ask_with_custom_default (message:&str, default_msg:&str, default:&String) -> String {
    let mut question = String::new();
    question = question.add(message);
    question = question.add(" (");
    question = question.add(default_msg);
    question = question.add("): ");

    println!("{}", question);

    let mut answer = String::new();
    io::stdin().read_line(&mut answer)
        .expect("Failed to read line");

    if WHITESPACE.is_match(&answer) {
        answer = default.clone();
    }

    answer.trim().to_string()
}
