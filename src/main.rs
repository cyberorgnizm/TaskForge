use std::io::{self, Write};

fn welcome() {
    const VERSION: f64 = 1.0;
    println!("Welcome to TaskForge! v{:.1} \n", VERSION);
    println!("Please select from the option provided:");
    println!("1. Add todo item.");
    println!("2. Edit todo item.");
    println!("3. Delete todo item.");
    println!("4. List todo items. \n");
    print!("=> ")
}

fn select_opt() -> String {
    let mut option = String::new();
    let _ = io::stdout().flush();
    io::stdin()
        .read_line(&mut option)
        .expect("Failed to read line.");

    return option;
}

fn main() {
    println!("Hello, world!");
}
