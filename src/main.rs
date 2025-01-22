use std::io::{self, Write};

fn welcome() {
    
    // App message and version scope
    {
        const VERSION: f64 = 1.0;
        let mut welcome_msg = String::from("Welcome to TaskForge!");
        welcome_msg.push_str(&format!("v{:.1}", VERSION));
        println!("{} \n", welcome_msg);

    }
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
    welcome();
    let option = select_opt().trim().parse().unwrap_or_else(|_| {
        // TODO loop until a valid option is selected
        println!("Invalid input. Please enter a number.");
        std::process::exit(1);
    });
    match option {
        1 => println!("Option 1 selected!"),
        2 => println!("Option 2 selected"),
        _ => println!("Invalid option!"),
    }
}
