use std::io::{self, Read};
use std::fs::File;

fn main() {
    println!("Welcome to the Tech Conference Mad Libs! Fill in the blanks below.");

    // Collect user inputs
    let adjective1 = get_input("Enter an adjective: ");
    let occupation = get_input("Enter an occupation: ");
    let buzzword = get_input("Enter a tech buzzword: ");
    let adjective2 = get_input("Enter another adjective: ");
    let verb_ing = get_input("Enter a verb ending in -ing: ");
    let liquid = get_input("Enter a type of liquid: ");
    let tech_item = get_input("Enter a piece of tech: ");
    let verb = get_input("Enter a verb: ");
    let random_object = get_input("Enter a random object: ");

    // Load the hidden story template from the file
    let mut file = File::open("story_template.txt").expect("Failed to open story file");
    let mut story_template = String::new();
    file.read_to_string(&mut story_template).expect("Failed to read story file");

    // Replace placeholders with user input
    let story = story_template
        .replace("{adjective1}", &adjective1)
        .replace("{occupation}", &occupation)
        .replace("{buzzword}", &buzzword)
        .replace("{adjective2}", &adjective2)
        .replace("{verb_ing}", &verb_ing)
        .replace("{liquid}", &liquid)
        .replace("{tech_item}", &tech_item)
        .replace("{verb}", &verb)
        .replace("{random_object}", &random_object);

    // Print the final story
    println!("\nHere’s your Mad Libs story:\n");
    println!("{}", story);
}

// Function to get user input
fn get_input(prompt: &str) -> String {
    let mut input = String::new();
    println!("{}", prompt);
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}
