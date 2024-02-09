use std::io::{self, Write};

struct Task {
    name: String,
    status: bool,
}

fn create_task(task_list: &mut Vec<Task>, task_name: String) {
    let new_task: Task = Task {
        name: task_name.clone(),
        status: false,
    };

    task_list.push(new_task);
}

fn list_task(task_list: &Vec<Task>) {
    println!("Task List:");
    for task in task_list {
        let status_str = if task.status {
            "Concluída"
        } else {
            "Pendente"
        };
        println!("{} - {}, ", task.name, status_str)
    }
}

fn remove_task(task_list: &Vec<Task>) {
 task.retain(|task| task.name != task.name);
}

fn main() {
    let mut task_list: Vec<Task> = Vec::new();

    loop {
        println!("\nMenu:");
        println!("1. Add Task");
        println!("2. List Task");
        print!("Enter your choice: ");

        io::stdout().flush().unwrap();

        let mut input: String = String::new();
        io::stdin().read_line(&mut input).expect("cannot readline");

        match input.trim() {
            "1" => {
                print!("Enter task description:");
                io::stdout().flush().unwrap();
                let mut name: String = String::new();
                io::stdin().read_line(&mut name).expect("cannot readline");

                create_task(&mut task_list, name.trim().to_string());
            }

            "2" => {
                list_task(&mut task_list);
            }

            _ => {
                println!("Wrong input.");
            }
        }
    }
}
