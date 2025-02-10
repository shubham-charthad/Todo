use ic_cdk_macros::{query, update};
use std::collections::VecDeque;
use std::io::{self, Write};

fn main() {
    let mut tasks: VecDeque<String> = VecDeque::new();

    loop {
        println!("\nTo-Do List:");
        println!("1. Add Task");
        println!("2. Remove Task");
        println!("3. View Tasks");
        println!("4. Exit");
        print!("Choose an option: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim();

        match choice {
            "1" => {
                print!("Enter task: ");
                io::stdout().flush().unwrap();
                let mut task = String::new();
                io::stdin().read_line(&mut task).unwrap();
                tasks.push_back(task.trim().to_string());
                println!("Task added.");
            }
            "2" => {
                if tasks.is_empty() {
                    println!("No tasks to remove.");
                } else {
                    println!("Enter task number to remove: ");
                    let mut index = String::new();
                    io::stdin().read_line(&mut index).unwrap();
                    if let Ok(idx) = index.trim().parse::<usize>() {
                        if idx > 0 && idx <= tasks.len() {
                            tasks.remove(idx - 1);
                            println!("Task removed.");
                        } else {
                            println!("Invalid task number.");
                        }
                    } else {
                        println!("Invalid input.");
                    }
                }
            }
            "3" => {
                if tasks.is_empty() {
                    println!("No tasks available.");
                } else {
                    println!("Your Tasks:");
                    for (i, task) in tasks.iter().enumerate() {
                        println!("{}. {}", i + 1, task);
                    }
                }
            }
            "4" => {
                println!("Exiting... Goodbye!");
                break;
            }
            _ => println!("Invalid choice, please try again."),
        }
    }
}
