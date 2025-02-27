use std::fs;

fn main() {
    let content = fs::read_to_string("example.txt").expect("Failed to read the file");
    println!("File content:\n{}", content);
}