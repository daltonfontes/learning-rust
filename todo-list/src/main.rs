use std::io;

struct Task {
    name: String,
    status: bool,
}

fn create_task(list: &mut Vec<Task>) {
    let mut task_name: String = String::new();
    io::stdin()
        .read_line(&mut task_name)
        .expect("cannot readline");

    let task: Task = Task {
        name: task_name,
        status: false,
    };

    list.push(task);
}

fn main() {
    let mut task_list: Vec<Task> = Vec::new();

    create_task(&mut task_list);

    for task in &task_list {
        println!("Name : {}, Status : {}", task.name, task.status);
    }
}
