use std::io;
use std::collections::HashMap;

/**
 * Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company.
 * For example, “Add Sally to Engineering” or “Add Amir to Sales.”
 * Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.
 */

fn main() {
    println!("Type a command to continue:");

    let mut directory = HashMap::new();

    loop {
        let mut command = String::new();
        io::stdin().read_line(&mut command).expect("failed to read line");

        let tokens: Vec<&str> = command
            .trim()
            .split_whitespace()
            .collect();

        match tokens.as_slice() {
            ["Add", person, "to", department] => {
                let listing = directory.entry(department.to_string()).or_insert(Vec::new());
                listing.push(person.to_string());
            }
            ["List", department] => {
                match directory.get(*department) {
                    Some(people) => println!("people in {}: {:?}", department, people),
                    None => println!("No people in {}!", department),
                }
            }
            ["exit"] => {
                println!("Exiting...");
                break;
            },
            _ => println!("Command not recognized. Type 'exit' to exit"),
        };
    }
}
