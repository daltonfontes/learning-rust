use std::{
    io::{self, Write},
    sync::atomic::{self, AtomicU64},
};

struct Task {
    id: u64,
    name: String,
    status: bool,
}
static UNIQUE_ID: AtomicU64 = AtomicU64::new(1);

fn create_task(task_list: &mut Vec<Task>, task_name: String) {
    let id_no = UNIQUE_ID.fetch_add(1, atomic::Ordering::SeqCst);
    let new_task: Task = Task {
        id: id_no,
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
        println!("({}) - {} - ({}), ", task.id, task.name, status_str);
    }
}

fn remove_task(task_list: &mut Vec<Task>, task_id: u64) {
    task_list.retain(|task| task.id != task_id);
}

fn update_task(task_list: &mut Vec<Task>, task_id: u64) {
    if let Some(task) = task_list.iter_mut().find(|task| task.id == task_id) {
        task.status = true;
    };
}

fn main() {
    let mut task_list: Vec<Task> = Vec::new();

    loop {
        println!("\nMenu:");
        println!("1. Add Task");
        println!("2. List Task");
        println!("3. Remove Task");
        println!("4. Update Task");
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

            "3" => {
                print!("Enter task 'ID' to remove: ");
                io::stdout().flush().unwrap();
                let mut task_id: String = String::new();
                io::stdin()
                    .read_line(&mut task_id)
                    .expect("cannot readline");
                if let Ok(task_id) = task_id.trim().parse::<u64>() {
                    remove_task(&mut task_list, task_id);
                }
            }

            "4" => {
                print!("Enter task ID to update 'Status': ");
                io::stdout().flush().unwrap();
                let mut task_id: String = String::new();
                io::stdin()
                    .read_line(&mut task_id)
                    .expect("cannot readline");

                update_task(&mut task_list, task_id.trim().parse::<u64>().unwrap());
            }

            _ => {
                println!("Wrong input.");
            }
        }
    }
}
