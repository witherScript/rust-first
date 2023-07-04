use std::{option, str::FromStr, sync::Arc};

struct Person {
    first_name: String,
    last_name: String,
    age: Option<u8>,
}

impl Person {
    fn new(first_name: String, last_name: String, age: Option<u8>) -> Person {
        Person {
            first_name,
            last_name,
            age,
        }
    }
    fn print(&self) {
        println!("Hello {} {}", self.first_name, self.last_name);
        if self.age.is_some() {
            println!("You are {} years old.", self.age.unwrap());
        }
    }
}

fn main() {
    let person = new_from_input();
    let person2 = Person::new("John".to_string(), "Doe".to_string(), Some(42));

    let new_vec = vec![person, person2];

    for number in 0..=1 {
        new_vec[number].print();
    }
}
fn new_from_input() -> Person {
    println!("What is your first name?");
    let first_name = read_string();
    println!("What is your last name?");
    let last_name = read_string();
    println!("What is your age?");
    let age = loop {
        match read_number() {
            Some(age) => break age,
            None => {
                println!("Please enter a valid age.");
                continue;
            }
        }
    };
    let person = Person::new(first_name, last_name, Some(age));
    person
}

fn read_string() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("can not read user input");
    let cleaned_input = input.trim().to_string();
    cleaned_input
}

fn read_number() -> Option<u8> {
    let input = read_string();
    u8::from_str(&input).ok()
}
