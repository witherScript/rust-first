use std::str::FromStr;
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

    match write_person(&person) {
        Ok(_) => println!("people.txt was written successfully"),
        Err(err) => println!("There was error while writing people.txt: {}", err),
    }

    match read_person() {
        Ok(person) => {
            println!("people.txt was read successfully");
            person.print();
        }
        Err(err) => println!("There was error while reading people.txt: {}", err),
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
            Some(age) => break Some(age),
            None => {
                println!("Please enter a valid age.");
                continue;
            }
        }
    };
    let person = Person::new(first_name, last_name, age);
    let _ = write_person(&person);
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

fn write_person(person: &Person) -> std::io::Result<()> {
    let mut output = String::new();
    output.push_str(&person.first_name);
    output.push('\n');
    output.push_str(&person.last_name);
    output.push('\n');
    if let Some(age) = person.age {
        output.push_str(&age.to_string());
    }
    output.push('\n');
    std::fs::write("people.txt", output)
}

fn read_person() -> Result<Person, std::io::Error> {
    let input = std::fs::read_to_string("people.txt")?;
    let mut lines = input.lines();
    let first_name = lines.next().unwrap_or("").to_string();
    let last_name = lines.next().unwrap_or("").to_string();
    let age_as_string = lines.next().unwrap_or("0").to_string();
    let age = u8::from_str(&age_as_string).ok();
    let person = Person {
        first_name,
        last_name,
        age,
    };
    Ok(person)
}
