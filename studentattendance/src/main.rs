use chrono::NaiveDate;
use std::io;

#[derive(Debug)]
struct Students {
    id: u32,
    name: String,
    class: String,
    section: String,
    is_present: bool,
    date: NaiveDate,
}

impl Students {
    fn new(id: u32, name: String, class: String, section: String, is_present: bool, date: NaiveDate) -> Self {
        Students {
            id,
            name,
            class,
            section,
            is_present,
            date,
        }
    }

    fn mark_attendance(&mut self, is_present: bool, date: NaiveDate) {
        self.is_present = is_present;
        self.date = date;
    }

    fn display(&self) {
        println!(
            "ID: {}, Name: {}, Class: {}, Section: {}, Present: {}, Date: {}",
            self.id,
            self.name,
            self.class,
            self.section,
            self.is_present,
            self.date
        );
    }
}

fn main() {
    let mut students = vec![
        Students::new(1, "Alice".to_string(), "10th".to_string(), "A".to_string(), false, chrono::Local::today().naive_local()),
        Students::new(2, "Bob".to_string(), "10th".to_string(), "A".to_string(), false, chrono::Local::today().naive_local()),
        Students::new(3, "Charlie".to_string(), "10th".to_string(), "A".to_string(), false, chrono::Local::today().naive_local()),
    ];
    let mut next_id = 4;

    loop {
        println!("Student Attendance System");
        println!("1. Add new student");
        println!("2. Mark attendance");
        println!("3. Display all students");
        println!("4. Quit");
        println!("Enter your choice:");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input, please enter a number.");
                continue;
            }
        };

        match choice {
            1 => {
                println!("Enter student name:");
                let mut name = String::new();
                io::stdin().read_line(&mut name).expect("Failed to read line");
                let name = name.trim().to_string();

                println!("Enter student class:");
                let mut class = String::new();
                io::stdin().read_line(&mut class).expect("Failed to read line");
                let class = class.trim().to_string();

                println!("Enter student section:");
                let mut section = String::new();
                io::stdin().read_line(&mut section).expect("Failed to read line");
                let section = section.trim().to_string();

                let date = chrono::Local::today().naive_local();
                let student = Students::new(next_id, name, class, section, false, date);
                students.push(student);
                next_id += 1;

                println!("Student added successfully!");
            }
            2 => {
                println!("Enter student ID to mark attendance:");
                let mut id_str = String::new();
                io::stdin().read_line(&mut id_str).expect("Failed to read line");
                let id: u32 = match id_str.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid ID, please enter a valid number.");
                        continue;
                    }
                };

                let student = students.iter_mut().find(|s| s.id == id);
                if let Some(student) = student {
                    println!("Enter 1 for present or 0 for absent:");
                    let mut status_str = String::new();
                    io::stdin().read_line(&mut status_str).expect("Failed to read line");
                    let is_present: bool = match status_str.trim() {
                        "1" => true,
                        "0" => false,
                        _ => {
                            println!("Invalid input, please enter 1 or 0.");
                            continue;
                        }
                    };

                    let date = chrono::Local::today().naive_local();
                    student.mark_attendance(is_present, date);
                    println!("Attendance marked successfully!");
                } else {
                    println!("Student with ID {} not found.", id);
                }
            }
            3 => {
                for student in &students {
                    student.display();
                }
            }
            4 => {
                println!("Goodbye!");
                break;
            }
            _ => {
                println!("Invalid choice, please enter a number between 1 and 4.");
            }
        }
    }
}
