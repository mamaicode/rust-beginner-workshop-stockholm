use std::io::{self, Write}; // Import standard input/output operations
use std::thread; // Import thread module to enable sleeping (pauses)
use std::time::Duration; // Import Duration to specify time intervals

// Function to handle countdown timer
fn countdown(seconds: u64, label: &str) {
    println!("\n{} starts!", label); // Announce the start of the session

    for remaining in (0..=seconds).rev() { // Loop from the given seconds down to zero
        print!("\rTime left: {:02}:{:02}", remaining / 60, remaining % 60); // Display time in MM:SS format
        io::stdout().flush().unwrap(); // Ensure output is displayed immediately
        
        thread::sleep(Duration::from_secs(1)); // Pause execution for 1 second
    }

    println!("\n{} finished!", label); // Notify when the session ends
}

fn main() {
    let pomodoro_duration = 60; // Set Pomodoro session to 1 minute
    let short_break = 30; // Set short break to 30 seconds
    let long_break = 60; // Set long break to 1 minute
    let mut session_count = 0; // Track number of completed sessions

    loop {
        countdown(pomodoro_duration, "Pomodoro Session"); // Start a Pomodoro session
        session_count += 1; // Increment session count

        // After every 4 Pomodoro sessions, take a long break
        if session_count % 2 == 0 {
            countdown(long_break, "Long Break");
        } else {
            countdown(short_break, "Short Break");
        }

        // Ask if the user wants to continue
        println!("Start another session? (y/n): ");
        let mut input = String::new(); // Create a buffer to store user input
        io::stdin().read_line(&mut input).unwrap(); // Read user input

        // If the user does not enter 'y', exit the loop
        if input.trim().to_lowercase() != "y" {
            println!("Good job! Pomodoro session ended."); // Display farewell message
            break; // Exit the loop
        }
    }
}
