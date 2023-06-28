struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    fn new(first_name: String, last_name: String) -> Person {
        Person {
            first_name,
            last_name,
        }
    }
    fn print(&self) {
        println!("Hello {} {}", self.first_name, self.last_name);
    }
}

fn main() {
    let person = new_from_input();
    let person2 = Person::new("John".to_string(), "Doe".to_string());

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
    let person = Person::new(first_name, last_name);
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
