// Starter code for the Rust Task Manager challenge

use std::fs::{File, OpenOptions};
use std::io::{self, Write, Read};
use std::path::Path;
use std::fmt;
use std::collections::HashMap;

// Task status enum
#[derive(Debug, Clone, PartialEq, Eq)]
enum TaskStatus {
    Pending,
    Completed,
}

impl fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TaskStatus::Pending => write!(f, "Pending"),
            TaskStatus::Completed => write!(f, "Completed"),
        }
    }
}

// Task struct to store task information
#[derive(Debug, Clone)]
struct Task {
    id: u32,
    title: String,
    description: String,
    due_date: Option<String>, // Consider using a proper date type in your implementation
    status: TaskStatus,
}

impl Task {
    fn display(&self) {
        println!("Task #{}: {}", self.id, self.title);
        println!("Status: {}", self.status);
        println!("Description: {}", self.description);
        if let Some(date) = &self.due_date {
            println!("Due date: {}", date);
        }
        println!();
    }

    fn to_string(&self) -> String {
        let status_str = match self.status {
            TaskStatus::Pending => "pending",
            TaskStatus::Completed => "completed",
        };
        
        let due_date_str = match &self.due_date {
            Some(date) => date.clone(),
            None => String::from("none"),
        };
        
        format!("{}|{}|{}|{}|{}", 
            self.id, 
            self.title, 
            self.description, 
            due_date_str,
            status_str
        )
    }

    fn from_string(s: &str) -> Option<Task> {
        let parts: Vec<&str> = s.split('|').collect();
        if parts.len() != 5 {
            return None;
        }

        let id = parts[0].parse::<u32>().ok()?;
        let title = parts[1].to_string();
        let description = parts[2].to_string();
        let due_date = if parts[3] == "none" {
            None
        } else {
            Some(parts[3].to_string())
        };
        let status = match parts[4] {
            "completed" => TaskStatus::Completed,
            _ => TaskStatus::Pending,
        };

        Some(Task {
            id,
            title,
            description,
            due_date,
            status,
        })
    }
}

// TaskManager to handle operations on tasks
struct TaskManager {
    tasks: Vec<Task>,
    next_id: u32,
}

impl TaskManager {
    // Create a new TaskManager
    fn new() -> TaskManager {
        TaskManager {
            tasks: Vec::new(),
            next_id: 1,
        }
    }
    
    // Add a new task to the task manager
    fn add_task(&mut self, title: String, description: String, due_date: Option<String>) -> &Task {
        let task = Task {
            id: self.next_id,
            title,
            description,
            due_date,
            status: TaskStatus::Pending,
        };
        
        self.next_id += 1;
        self.tasks.push(task);
        self.tasks.last().unwrap()
    }
    
    // List all tasks
    fn list_tasks(&self) {
        if self.tasks.is_empty() {
            println!("No tasks found.");
            return;
        }

        println!("Task List:");
        println!("----------");
        for task in &self.tasks {
            task.display();
        }
    }
    
    // Find a task by ID
    fn find_task(&self, id: u32) -> Option<&Task> {
        self.tasks.iter().find(|task| task.id == id)
    }
    
    // Find a task by ID (mutable version)
    fn find_task_mut(&mut self, id: u32) -> Option<&mut Task> {
        self.tasks.iter_mut().find(|task| task.id == id)
    }
    
    // Mark a task as complete
    fn complete_task(&mut self, id: u32) -> Result<&Task, &'static str> {
        let task = self.find_task_mut(id).ok_or("Task not found")?;
        task.status = TaskStatus::Completed;
        Ok(task)
    }
    
    // Delete a task
    fn delete_task(&mut self, id: u32) -> Result<(), &'static str> {
        let position = self.tasks.iter().position(|task| task.id == id).ok_or("Task not found")?;
        self.tasks.remove(position);
        Ok(())
    }
    
    // Filter tasks by status
    fn filter_by_status(&self, status: &TaskStatus) -> Vec<&Task> {
        self.tasks.iter()
            .filter(|task| task.status == *status)
            .collect()
    }
    
    // Filter tasks by due date (simple string comparison)
    fn filter_by_due_date(&self, due_date: &str) -> Vec<&Task> {
        self.tasks.iter()
            .filter(|task| {
                if let Some(date) = &task.due_date {
                    date == due_date
                } else {
                    false
                }
            })
            .collect()
    }
    
    // Save tasks to a file
    fn save_to_file(&self, filename: &str) -> io::Result<()> {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(filename)?;
        
        // Save the next_id at the top of the file
        writeln!(file, "{}", self.next_id)?;
        
        // Save each task
        for task in &self.tasks {
            writeln!(file, "{}", task.to_string())?;
        }
        
        println!("Tasks saved to '{}'", filename);
        Ok(())
    }
    
    // Load tasks from a file
    fn load_from_file(&mut self, filename: &str) -> io::Result<()> {
        if !Path::new(filename).exists() {
            return Err(io::Error::new(io::ErrorKind::NotFound, "File not found"));
        }
        
        let mut file = File::open(filename)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        
        let lines: Vec<&str> = contents.lines().collect();
        if lines.is_empty() {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "File is empty"));
        }
        
        // First line is the next_id
        self.next_id = lines[0].parse::<u32>().unwrap_or(1);
        
        // Clear existing tasks
        self.tasks.clear();
        
        // Load tasks
        for line in &lines[1..] {
            if let Some(task) = Task::from_string(line) {
                self.tasks.push(task);
            }
        }
        
        println!("Loaded {} tasks from '{}'", self.tasks.len(), filename);
        Ok(())
    }
    
    // Generate statistics about tasks
    fn generate_stats(&self) -> HashMap<String, usize> {
        let mut stats = HashMap::new();
        
        stats.insert("total".to_string(), self.tasks.len());
        stats.insert("pending".to_string(), 
            self.tasks.iter().filter(|t| t.status == TaskStatus::Pending).count());
        stats.insert("completed".to_string(), 
            self.tasks.iter().filter(|t| t.status == TaskStatus::Completed).count());
            
        // Count tasks by due date
        let mut dates = HashMap::new();
        for task in &self.tasks {
            if let Some(date) = &task.due_date {
                *dates.entry(date.clone()).or_insert(0) += 1;
            }
        }
        
        stats
    }
}

// Command enum to represent user commands
enum Command {
    Add { title: String, description: String, due_date: Option<String> },
    List,
    ListPending,
    ListCompleted,
    Complete { id: u32 },
    Delete { id: u32 },
    Save { filename: String },
    Load { filename: String },
    Stats,
    Help,
    Quit,
    Unknown,
}

// Parse a command from user input
fn parse_command(input: &str) -> Command {
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    
    if parts.is_empty() {
        return Command::Unknown;
    }
    
    match parts[0].to_lowercase().as_str() {
        "add" => {
            if parts.len() < 3 {
                println!("Error: 'add' command requires a title and description");
                return Command::Unknown;
            }
            let title = parts[1].to_string();
            let description = parts[2].to_string();
            let due_date = if parts.len() > 3 {
                Some(parts[3].to_string())
            } else {
                None
            };
            Command::Add { title, description, due_date }
        },
        "list" => Command::List,
        "pending" => Command::ListPending,
        "completed" => Command::ListCompleted,
        "complete" => {
            if parts.len() < 2 {
                println!("Error: 'complete' command requires a task ID");
                return Command::Unknown;
            }
            match parts[1].parse::<u32>() {
                Ok(id) => Command::Complete { id },
                Err(_) => {
                    println!("Error: Task ID must be a positive number");
                    Command::Unknown
                }
            }
        },
        "delete" => {
            if parts.len() < 2 {
                println!("Error: 'delete' command requires a task ID");
                return Command::Unknown;
            }
            match parts[1].parse::<u32>() {
                Ok(id) => Command::Delete { id },
                Err(_) => {
                    println!("Error: Task ID must be a positive number");
                    Command::Unknown
                }
            }
        },
        "save" => {
            if parts.len() < 2 {
                println!("Error: 'save' command requires a filename");
                return Command::Unknown;
            }
            Command::Save { filename: parts[1].to_string() }
        },
        "load" => {
            if parts.len() < 2 {
                println!("Error: 'load' command requires a filename");
                return Command::Unknown;
            }
            Command::Load { filename: parts[1].to_string() }
        },
        "stats" => Command::Stats,
        "help" => Command::Help,
        "quit" | "exit" => Command::Quit,
        _ => Command::Unknown,
    }
}

fn display_help() {
    println!("Task Manager - Available Commands:");
    println!("----------------------------------");
    println!("add <title> <description> [due_date] - Add a new task");
    println!("list - List all tasks");
    println!("pending - List pending tasks");
    println!("completed - List completed tasks");
    println!("complete <id> - Mark a task as completed");
    println!("delete <id> - Delete a task");
    println!("save <filename> - Save tasks to a file");
    println!("load <filename> - Load tasks from a file");
    println!("stats - Show task statistics");
    println!("help - Display this help message");
    println!("quit - Exit the program");
    println!();
}

fn main() {
    // Initialize task manager
    let mut task_manager = TaskManager::new();
    
    println!("Welcome to the Rust Task Manager!");
    println!("Type 'help' for a list of commands.");
    
    // Main application loop
    loop {
        // Get user input
        print!("> ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        
        let command = parse_command(&input);
        
        // Process command
        match command {
            Command::Add { title, description, due_date } => {
                let task = task_manager.add_task(title, description, due_date);
                println!("Task added with ID: {}", task.id);
            },
            Command::List => {
                task_manager.list_tasks();
            },
            Command::ListPending => {
                let pending_tasks = task_manager.filter_by_status(&TaskStatus::Pending);
                if pending_tasks.is_empty() {
                    println!("No pending tasks found.");
                } else {
                    println!("Pending Tasks:");
                    println!("--------------");
                    for task in pending_tasks {
                        task.display();
                    }
                }
            },
            Command::ListCompleted => {
                let completed_tasks = task_manager.filter_by_status(&TaskStatus::Completed);
                if completed_tasks.is_empty() {
                    println!("No completed tasks found.");
                } else {
                    println!("Completed Tasks:");
                    println!("----------------");
                    for task in completed_tasks {
                        task.display();
                    }
                }
            },
            Command::Complete { id } => {
                match task_manager.complete_task(id) {
                    Ok(_) => println!("Task #{} marked as completed", id),
                    Err(e) => println!("Error: {}", e),
                }
            },
            Command::Delete { id } => {
                match task_manager.delete_task(id) {
                    Ok(_) => println!("Task #{} deleted", id),
                    Err(e) => println!("Error: {}", e),
                }
            },
            Command::Save { filename } => {
                match task_manager.save_to_file(&filename) {
                    Ok(_) => {},
                    Err(e) => println!("Error saving to file: {}", e),
                }
            },
            Command::Load { filename } => {
                match task_manager.load_from_file(&filename) {
                    Ok(_) => {},
                    Err(e) => println!("Error loading from file: {}", e),
                }
            },
            Command::Stats => {
                let stats = task_manager.generate_stats();
                println!("Task Statistics:");
                println!("----------------");
                println!("Total tasks: {}", stats.get("total").unwrap_or(&0));
                println!("Pending tasks: {}", stats.get("pending").unwrap_or(&0));
                println!("Completed tasks: {}", stats.get("completed").unwrap_or(&0));
                println!();
            },
            Command::Help => {
                display_help();
            },
            Command::Quit => {
                println!("Goodbye!");
                break;
            },
            Command::Unknown => {
                println!("Unknown command. Type 'help' for a list of commands.");
            },
        }
    }
}