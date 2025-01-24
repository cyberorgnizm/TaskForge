use std::io::{self, Write};

#[derive(Debug)]
struct Task {
    title: String,
    is_completed: bool
}

impl Task {
    fn complete(&mut self) -> bool {
        self.is_completed = true;
        return  self.is_completed
    }
}

fn new_task(title: String, is_completed: bool) -> Task {
    Task {
        title,
        is_completed
    }
}

fn welcome() {
    // App message and version scope
    {
        const VERSION: f64 = 1.0;
        let mut welcome_msg = String::from("Welcome to TaskForge!");
        welcome_msg.push_str(&format!(" v{:.1}", VERSION));
        println!("{} \n", welcome_msg);

    }
    println!("Please select from the option provided:");
    println!("1. Add todo item.");
    println!("2. Edit todo item.");
    println!("3. Delete todo item.");
    println!("4. List todo items. \n");
    print!("=> ")
}

fn select_opt(option: &mut String) {
    let _ = io::stdout().flush();
    io::stdin()
        .read_line(option)
        .expect("Failed to read line.");
}

fn main() {
    welcome();
    let mut option = String::new();
    select_opt(&mut option);
    
    let option = option.trim().parse().unwrap_or_else(|_| {
        // TODO loop until a valid option is selected
        println!("Invalid input. Please enter a number.");
        std::process::exit(1);
    });

    match option {
        1 => println!("Option 1 selected!"),
        2 => println!("Option 2 selected!"),
        3 => println!("Option 3 selected!"),
        4 => println!("Option 4 selected!"),
        _ => println!("Invalid option!"),
    }
}
