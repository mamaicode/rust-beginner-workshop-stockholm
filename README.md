# Rust Workshop for Beginners

DevFest Sweden Rustacean workshop, Feb 27, 2025
Introductory workshop to Rust as a programming language.


### 1. Introduction to Rust (15 min)
- **What is Rust? Why is it popular?**
- **Key features:** Safety, performance, concurrency
- **Setting up Rust:** `rustup`, `cargo`, `rustc`
- **Creating a new Rust project:**

### 2. Writing Your First Rust Program (20 min)
- Explaining the `main.rs` structure
- Printing to the console: `println!()`
- Variables and mutability (`let`, `mut`)
- Basic data types (`String`, `u32`, `bool`)
- Taking user input with `std::io::stdin()`
- **Project:** Mad Libs CLI

### 3. Implementing Logic in the CLI App (30 min)
- Basic error handling (`expect`, `match`, `Result`)
- Writing simple functions to process data
- **Project:** Pomodoro App

### 4. Compiling Rust Apps (10 min)
- Debugging tips
- Adding dependencies via `Cargo.toml`

### 5. Q&A + Next Steps (15 min)
- Encouraging continued learning
- Recommended resources

***
## Writing Your First Rust Program
When you create a new Rust project with Cargo (`cargo new my_project`), it generates a `src/main.rs` file. This is the entry point of your Rust application.

```rust
fn main() {
    println!("Hello, Rust!");
}

    fn main(): This defines the main function, which is the starting point of the program.
    println!(): This is a macro (indicated by !) used to print output to the console.
```

Run your program with: `cargo run`


### Printing to the Console: println!()
Rust uses println!() to display text.

```rust
fn main() {
    println!("Hello, world!"); // Simple text output
    println!("The answer is: {}", 42); // Using placeholders
    println!("{} is {} years old.", "Alice", 25); // Multiple placeholders
}
```

{} is a placeholder that gets replaced with values.
You can pass multiple values separated by commas.

### What is a Macro in Rust?

A macro in Rust is a way to write code that writes other code (metaprogramming). Macros allow you to generate repetitive or complex code at compile time, making your programs more flexible and reducing boilerplate.

The println!() macro is used to print text to the console:

```rust
fn main() {
    println!("Hello, Rust!");
    println!("My age is {}", 25);
}
```
println!() expands to a function call at compile time, formatting the string dynamically.

<br>

Rust allows you to create custom macros using `macro_rules!`.


```rust
macro_rules! say_hello {
    () => {
        println!("Hello, DevFest Stockholm!");
    };
}

fn main() {
    say_hello!(); // Expands to println!("Hello, DevFest Stockholm!");
}
```

### Explaining the println! Macro in Rust

The println! macro in Rust is a built-in macro used for printing formatted output to the console, followed by a newline. 

Let's break down its definition:
```rust
macro_rules! println {
    () => {
        $crate::print!("\n")
    };
    ($($arg:tt)*) => {{
        $crate::io::_print($crate::format_args_nl!($($arg)*));
    }};
}
```

#### Understanding the Macro Structure

This macro has two arms (or cases):

##### First Arm  `(() => { $crate::print!("\n") };)`

If `println!()` is called without arguments, it simply prints a newline `("\n")`.  
It expands to `print!("\n")`, which prints an empty line without formatting.  
`$crate::print!("\n")` ensures the macro resolves correctly within Rust's standard library.

##### Second Arm `(($($arg:tt)*) => {{ ... }})`

This is used when `println!` is called with arguments, like `println!("Hello, {}!", "world")`.  
`$($arg:tt)*` is a macro pattern that captures all arguments passed to `println!`.  
The captured arguments are passed to `$crate::format_args_nl!($($arg)*)`, which:  
- Processes formatting.  
- Ensures a newline is added automatically.  

The result is then passed to `$crate::io::_print(...)`, which performs the actual output operation.


##### So then... What is $crate?

    $crate is a special Rust macro variable that refers to the current crate (module) where the macro is defined.
    It ensures that the macro always resolves correctly, even if the standard library is used in different contexts.

For example, instead of:  
`std::io::_print(std::format_args_nl!("Hello, Rust!"));`  

The macro expands to:  
`$crate::io::_print($crate::format_args_nl!("Hello, Rust!"));`  

This makes `println!` reliable in any Rust environment.
***
## Variables and Mutability in Rust (let, mut)

In Rust, variables are immutable by default, meaning their values cannot be changed after they are assigned. However, Rust allows mutability using the mut keyword.

```rust
fn main() {
    let x = 5;  // Immutable variable
    println!("The value of x is: {}", x);
    
    // x = 10; // ❌ This would cause an error: "cannot assign twice to immutable variable"
}
```
let `x = 5;` declares an immutable variable named `x`.
If you try to change `x` later, Rust will throw an error before even compiling.

### Making a Variable Mutable with mut
```rust
fn main() {
    let mut y = 10;  // Mutable variable
    println!("Initial value of y: {}", y);
    
    y = 20;  // ✅ Allowed because y is mutable
    println!("Updated value of y: {}", y);
}
```
`let mut y = 10;` declares `y` as mutable, meaning it can be changed.
`y = 20;` successfully updates `y` because we used mut.
Without mut, this reassignment would cause an error.

#### Why Are Variables Immutable by Default?

Rust enforces immutability by default for several reasons: 

- ✅ Safety: Prevents accidental changes to data.
- ✅ Concurrency: Immutable data can be safely shared between threads.
- ✅ Optimization: The compiler can optimize immutable variables better.

If you need to modify a variable, you must explicitly declare it as mut.
***
## Basic data types (String, u32, bool)
Rust is a statically typed language, meaning every variable has a specific type known at compile time. Here, we'll focus on three fundamental types: `String`, `u32`, and `bool`.
String Types in Rust
### String
Rust has two primary ways to store text:

- String literals (&str) – Fixed-length text, stored in binary.
- String type – A growable heap-allocated string.

#### String Literal (&str)

```rust
fn main() {
    let name: &str = "Irine"; // Immutable fixed string
    println!("Hello, {}!", name);
}
```
Here, `&str` is a string slice, which is immutable and stored directly in memory. It is fast but cannot be modified.

#### Growable String Type

```rust
fn main() {
    let mut message = String::from("Hello");
    message.push_str(", Rust!"); // Appends ", Rust!"
    println!("{}", message);
}
```
`String::from()` creates a heap-allocated, growable string.
`.push_str()` appends text.

#### 💡 Use String when modifying text, and &str for fixed values.
***
### u32 – Unsigned 32-bit Integer

Rust has multiple integer types, but u32 is a common choice for positive numbers.
```rust
fn main() {
    let age: u32 = 24;
    println!("Age: {}", age);
}
```
- `u32` means "unsigned 32-bit integer" (0 to 4,294,967,295).
- Unsigned (u prefix) → No negative numbers.
- 32-bit → Can store numbers up to ~4.2 billion.

##### Common Integer Types in Rust
| Type  | Description                 | Range               |
|-------|-----------------------------|----------------------|
| `i8`  | Signed 8-bit integer        | -128 to 127         |
| `u8`  | Unsigned 8-bit integer      | 0 to 255            |
| `i32` | Signed 32-bit integer (default) | ~-2B to 2B     |
| `u32` | Unsigned 32-bit integer     | 0 to ~4.2B          |
| `i64` | Signed 64-bit integer       | ~±9 quintillion     |

#### 💡 Use u32 when you need a positive whole number and want to save memory.
***
### `bool` – True or False Values

A `bool` type represents a value that is either true or false.
```rust
fn main() {
    let is_rust_fun: bool = true;
    println!("Is Rust fun? {}", is_rust_fun);
}
```
- true and false are the only possible values.
- Used in logic, conditions, and flags.

Boolean in Conditional Statements
```rust
fn main() {
    let is_participant = false;

    if is_participant {
        println!("Welcome, workshop participant!");
    } else {
        println!("Access denied.");
    }
}
```
`if` checks whether is_admin is true or false.
***
### Taking user input with `std::io::stdin()`
Rust reads user input using std::io::stdin(), which allows the program to get input from the keyboard.

Let's write a simple program that asks for the user's name and prints it.
```rust
use std::io; // Import the input/output module

fn main() {
    let mut name = String::new(); // Create an empty String to store input

    println!("Enter your name:");  
    io::stdin().read_line(&mut name).expect("Failed to read input");

    println!("Hello, {}!", name.trim()); // `.trim()` removes the newline
}
```

How It Works
- use `std::io;` → Import the I/O module (required for user input).
- let `mut name = String::new();` → Create a mutable empty String to store input.
- `io::stdin().read_line(&mut name)` → Reads user input and stores it in name.
- `.expect("Failed to read input");` → Handles potential errors.
- `.trim()` → Removes extra whitespace and newline characters from input.

#### Taking Numerical Input (u32)
Rust stores input as a string, so we need to convert it to a number.
```rust
use std::io;

fn main() {
    let mut age = String::new();

    println!("Enter your age:");
    io::stdin().read_line(&mut age).expect("Failed to read input");
 
    let age: u32 = age.trim().parse().expect("Please enter a valid number");

    println!("You are {} years old.", age);
}
```
What’s Happening?
- `String::new()` stores the input as text.
- `.trim()` removes extra spaces/newlines.
- `.parse::<u32>()` converts the string into a `u32` integer.
- `.expect("Please enter a valid number")` ensures valid input.

If the user enters text instead of a number, the program will panic (crash).

#### Handling Invalid Input Gracefully
Let's handle errors properly so the program doesn't crash.
```
use std::io;

fn main() {
    let mut input = String::new();

    println!("Enter a number:");
    io::stdin().read_line(&mut input).expect("Failed to read input");

    match input.trim().parse::<u32>() {
        Ok(num) => println!("You entered: {}", num),
        Err(_) => println!("That's not a valid number!"),
    }
}
```
How This Works
- `match` checks if parsing is successful `(Ok(num))` or `fails (Err(_))`.
- If input is valid, it prints the number.
- If input is invalid, it shows an error without crashing.
***
## Implementing Logic in the CLI App
### Error handling
 
Error handling in Rust is based on the `Result<T, E>` and `Option<T>` types, along with constructs like `expect`, `match`, and the `?` operator. Below are some beginner-friendly explanations with examples.

#### Using expect() for Quick Error Handling

`expect()` is a quick way to handle errors. If an operation fails, it will print an error message and panic (crash the program).
Example: Using expect() to Read a File
```rust
use std::fs;

fn main() {
    let content = fs::read_to_string("example.txt").expect("Failed to read the file");
    println!("File content:\n{}", content);
}
```
- 🔹 If "example.txt" doesn't exist, Rust will panic and print:
- thread 'main' panicked at 'Failed to read the file: No such file or directory'

#### Using match to Handle Errors Gracefully

`match` allows you to explicitly handle success `(Ok)` and `failure (Err)` cases.
Example: Parsing a Number with match
```rust
fn main() {
    let input = "42a";  // Invalid number

    match input.parse::<i32>() {
        Ok(num) => println!("Parsed number: {}", num),
        Err(e) => println!("Error parsing number: {}", e),
    }
}
```

-🔹 This prevents a crash and prints:
Error parsing number: invalid digit found in string

-✅ Best for: When you need fine-grained error handling.

#### Using ? Operator for Simpler Error Propagation

Instead of handling every error with match, the ? operator propagates errors automatically.
Example: Simplifying read_file()

```rust
use std::fs;
use std::io;

fn read_file(path: &str) -> Result<String, io::Error> {
    fs::read_to_string(path) // Automatically returns `Err` if it fails
}

fn main() {
    match read_file("example.txt") {
        Ok(content) => println!("File content:\n{}", content),
        Err(e) => println!("Error reading file: {}", e),
    }
}
```

- 🔹 ? makes error handling shorter and cleaner.

- ✅ Best for: Making functions readable while allowing errors to be handled by the caller.
***