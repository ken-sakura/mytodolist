use std::fs::{File, OpenOptions};
use std::io::{stdin, stdout, Write, BufRead, BufReader};

#[derive(Debug)]
struct Todo {
    task: String,
    completed: bool,
}

impl Todo {
    fn new(task: String) -> Todo {
        Todo {
            task,
            completed: false,
        }
    }
}

fn load_todos(filename: &str) -> Vec<Todo> {
    let file = match File::open(filename) {
        Ok(file) => file,
        Err(_) => return Vec::new(), // ファイルが存在しない場合は空のベクタを返す
    };
    let reader = BufReader::new(file);
    let mut todos = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let parts: Vec<&str> = line.split('|').collect();
        if parts.len() == 2 {
            let task = parts[0].to_string();
            let completed = parts[1] == "true";
            todos.push(Todo { task, completed });
        }
    }

    todos
}

fn save_todos(filename: &str, todos: &Vec<Todo>) {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(filename)
        .unwrap();

    for todo in todos {
        let line = format!("{}|{}\n", todo.task, todo.completed);
        file.write_all(line.as_bytes()).unwrap();
    }
}

fn add_todo(todos: &mut Vec<Todo>, filename: &str) {
    print!("Enter task: ");
    stdout().flush().unwrap();

    let mut task = String::new();
    stdin().read_line(&mut task).expect("Failed to read line");
    task = task.trim().to_string();

    let todo = Todo::new(task);
    todos.push(todo);
    save_todos(filename, todos);
    println!("Task added!");
}

fn list_todos(todos: &Vec<Todo>) {
    if todos.is_empty() {
        println!("No tasks in the list!");
        return;
    }

    for (index, todo) in todos.iter().enumerate() {
        println!("{}: {} [{}]", index + 1, todo.task, if todo.completed { "Done" } else { "Pending" });
    }
}

fn complete_todo(todos: &mut Vec<Todo>, filename: &str) {
    list_todos(todos);
    if todos.is_empty() {
        return;
    }

    print!("Enter the number of the task to complete: ");
    stdout().flush().unwrap();

    let mut index = String::new();
    stdin().read_line(&mut index).expect("Failed to read line");
    let index: usize = match index.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter a number.");
            return;
        },
    };

    if index > 0 && index <= todos.len() {
        todos[index - 1].completed = true;
        save_todos(filename, todos);
        println!("Task marked as completed!");
    } else {
        println!("Invalid task number.");
    }
}

fn remove_todo(todos: &mut Vec<Todo>, filename: &str) {
    list_todos(todos);
    if todos.is_empty() {
        return;
    }

    print!("Enter the number of the task to remove: ");
    stdout().flush().unwrap();

    let mut index = String::new();
    stdin().read_line(&mut index).expect("Failed to read line");
    let index: usize = match index.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter a number.");
            return;
        },
    };

    if index > 0 && index <= todos.len() {
        todos.remove(index - 1);
        save_todos(filename, todos);
        println!("Task removed!");
    } else {
        println!("Invalid task number.");
    }
}

fn main() {
    let filename = "todos.txt";
    let mut todos = load_todos(filename);

    loop {
        println!("\nChoose an action:");
        println!("1. Add task");
        println!("2. List tasks");
        println!("3. Complete task");
        println!("4. Remove task");
        println!("5. Exit");

        print!("Enter your choice: ");
        stdout().flush().unwrap();

        let mut choice = String::new();
        stdin().read_line(&mut choice).expect("Failed to read line");

        match choice.trim() {
            "1" => add_todo(&mut todos, filename),
            "2" => list_todos(&todos),
            "3" => complete_todo(&mut todos, filename),
            "4" => remove_todo(&mut todos, filename),
            "5" => {
                println!("See you !!");
                break;
            },
            _ => println!("Invalid choice, try again."),
        }
    }
}