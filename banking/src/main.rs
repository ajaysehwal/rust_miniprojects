use std::io;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct Account{
    id:u32,
    name:String,
    balance:f64,
    valid_signature: String
}

impl Account{
    fn new(id:u32,name:String,valid_signature:String)->Self{
        Account{
            id,name,balance:0.0,valid_signature
        }
    }
    fn deposit(&mut self,amount:f64,signature:String)->Result<(),String>{

         if self.validate_signature(&signature){
            self.balance+=amount;
            Ok(())

         } else {
            Err("Invalid signature".to_string())
         }
        
    }
    fn withdraw(&mut self,amount:f64,signature:String)->Result<(),String>{
        if self.validate_signature(&signature) {
            if self.balance>=amount{
                self.balance-=amount;
                Ok(())
            }else{
                Err("Insufficient funds".to_string())
            }
        }else{
            Err("Invalid Signature".to_string())
        }
       
    }
    fn validate_signature(&self,signature:&str)->bool{
       return self.valid_signature==signature;
    }
}


fn main(){
    let mut accounts: Vec<Account> = Vec::new();
 
    loop{
        println!("1. Open a new account");
        println!("2. Select an account");
        println!("3. Quit");
        println!("Enter your choice:");
        // let mut account: Account=Account::new(1, "Alice".to_string(), "alicesign".to_string());
        let mut choice=String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match choice{
            1=>{
                println!("Enter your full name");
                let mut name=String::new();
                io::stdin().read_line(&mut name).expect("failed to read line");
                let name=name.trim().to_string();
                println!("Enter a valid signature for the account:");
                let mut signature = String::new();
                io::stdin().read_line(&mut signature).expect("Failed to read line");
                let signature = signature.trim().to_string();
                let id=(accounts.len()+1)as u32;
                let new_account=Account::new(id, name, signature);
                accounts.push(new_account);
                println!("Account created successfully");

            }
            2=>{
                if accounts.is_empty(){
                    println!("No accounts available. Please create an account first.");
                    continue;
                }
                println!("Available accounts");
                for account in &accounts{
                    println!("ID: {}, Name: {}", account.id, account.name)
                }
                println!("Enter account Id to select");
                let mut id_str=String::new();
                io::stdin().read_line(&mut id_str).expect("Failed to read line");
                let id:u32=match id_str.trim().parse() {
                    Ok(num)=>num,
                    Err(_)=>{
                        println!("Invalid ID, please enter a valid number.");
                        continue;
                    }
                };
                let selected_account=accounts.iter_mut().find(|acc| acc.id==id);
                if let Some(account)= selected_account{
                    account_menu(account);
                }else {
                    println!("Account with ID {} not found.", id);
                }
                
            }
            3 => {
                println!("Goodbye!");
                break;
            }
            _ => {
                println!("Invalid choice, please enter a number between 1 and 3.");
            }
    }
   
    }
}

fn account_menu(account: &mut Account) {
    loop {
        println!("Account Menu for {}: ", account.name);
        println!("1. Deposit");
        println!("2. Withdraw");
        println!("3. Check Balance");
        println!("4. Back to Main Menu");
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
                println!("Enter the amount to deposit:");
                let mut amount_str = String::new();
                io::stdin().read_line(&mut amount_str).expect("Failed to read line");
                let amount: f64 = match amount_str.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid amount, please enter a valid number.");
                        continue;
                    }
                };

                println!("Enter your signature:");
                let mut signature = String::new();
                io::stdin().read_line(&mut signature).expect("Failed to read line");
                let signature = signature.trim().to_string();

                match account.deposit(amount, signature) {
                    Ok(()) => println!("Deposit successful! New balance: {}", account.balance),
                    Err(e) => println!("Deposit failed: {}", e),
                }
            }
            2 => {
                println!("Enter the amount to withdraw:");
                let mut amount_str = String::new();
                io::stdin().read_line(&mut amount_str).expect("Failed to read line");
                let amount: f64 = match amount_str.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid amount, please enter a valid number.");
                        continue;
                    }
                };

                println!("Enter your signature:");
                let mut signature = String::new();
                io::stdin().read_line(&mut signature).expect("Failed to read line");
                let signature = signature.trim().to_string();

                match account.withdraw(amount, signature) {
                    Ok(()) => println!("Withdrawal successful! New balance: {}", account.balance),
                    Err(e) => println!("Withdrawal failed: {}", e),
                }
            }
            3 => {
                println!("Current balance: {}", account.balance);
            }
            4 => {
                break;
            }
            _ => {
                println!("Invalid choice, please enter a number between 1 and 4.");
            }
        }
    }
}