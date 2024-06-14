use std::io;

struct Task{
    id:u32,
    description:String,
    completed:bool,
}
struct TodoList{
    tasks:Vec<Task>,
    next_id:u32,
}

impl TodoList{
    fn new()->Self{
        TodoList{
            tasks:Vec::new(),
            next_id:1,
        }
    }
    fn add_task(&mut self,description:String){
        let task=Task{
            id:self.next_id,
            description,
            completed:false
        };
        self.tasks.push(task);
        self.next_id+=1;
    }
    fn remove_task(&mut self,id:u32){
        self.tasks.retain(|task|task.id!=id)
    }
    fn list_tasks(&self){
        for task in &self.tasks{
            println!("{}:{}[{}]",task.id,task.description,if task.completed{"X"}else {""});
        }
    }
    fn mark_task_completed(&mut self,id:u32){
        if let Some(task)=self.tasks.iter_mut().find(|task| task.id==id){
            task.completed=true
        }
    }
}

fn main(){
   let mut todo_list=TodoList::new();
   loop{
    println!("1. Add task");
    println!("2. Remove task");
    println!("3. List tasks");
    println!("4. Mark task as completed");
    println!("5. Quit");
    println!("Enter your choice:");
    let mut choice=String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read line");
    let choice: u32 = match choice.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };
    match choice{
        1=>{
            println!("Enter task descriptions");
            let mut description=String::new();
            io::stdin().read_line(&mut description).expect("Failed to read line");
            todo_list.add_task(description);
         }
         2=>{
            println!("Enter task ID to remove:");
            let mut id = String::new();
            io::stdin().read_line(&mut id).expect("Failed to read line");
            let id: u32 = match id.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };
            todo_list.remove_task(id);
         }
         3 => {
            todo_list.list_tasks();
        }
        4 => {
            println!("Enter task ID to mark as completed:");
            let mut id = String::new();
            io::stdin().read_line(&mut id).expect("Failed to read line");
            let id: u32 = match id.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };
            todo_list.mark_task_completed(id);
        }
        5 => break,
        _ => continue,
    }
    
   } 
}