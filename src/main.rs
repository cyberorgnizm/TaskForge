use std::io::{self, Write};

#[derive(Debug)]
struct Task {
    id: u32,
    title: String,
    is_completed: bool
}

impl Task {
    fn complete(&mut self) -> bool {
        self.is_completed = true;
        return  self.is_completed
    }
}

fn new_task(id: u32, title: String, is_completed: bool) -> Task {
    Task {
        id,
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

fn add_todo(tasks: &mut Vec<Task>) {
    print!("Enter the name of the task: ");
    let _ = io::stdout().flush();
    let mut title = String::new();
    io::stdin()
        .read_line(&mut title)
        .expect("Failed to read line.");
    let total_tasks = tasks.len() as u32;
    let mut task = new_task(total_tasks + 1, title, false);

    println!("Todo Item created!");
    println!("{}. {}", task.id, task.title);
    tasks.push(task);
}

fn edit_todo(tasks: &mut Vec<Task>) {
    println!("Enter the ID of the task to edit from the list below:");
    for task in tasks {
        println!("{}. {}", task.id, task.title);
    }

    let mut selected_id = String::new();

    io::stdin()
    .read_line(&mut selected_id)
    .expect("Error reading line");

    let selected_id = selected_id.parse().expect("Not a valid number");

    let task = tasks.iter_mut().filter(|t| t.id == selected_id);

    // println!("Mark as completed(Y/N): {}", task.is_completed);

    // let resp = String::new();

    // let resp = resp.trim();

    // if resp == "Y" {
    //     task.complete();
    // }
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

    let mut task_list = Vec::new();

    match option {
        1 => add_todo(&mut task_list),
        2 => edit_todo(&mut task_list),
        3 => println!("Option 3 selected!"),
        4 => println!("Option 4 selected!"),
        _ => println!("Invalid option!"),
    }
}
