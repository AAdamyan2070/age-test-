use std::cmp::Ordering;
use std::io;

fn main() {
    let oldest_person_age: u32 = 6000000;
    
    println!("Welcome to the game where you can test if your older than the oldest guy on earth !!!");

    loop{
    
        println!("Please enter your age: ");
        let mut users_age = String::new();
        io::stdin().read_line(&mut users_age).expect("err");
        
        let users_age: u32 = match users_age.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match users_age.cmp(&oldest_person_age) {
            Ordering::Less => {
                println!("{users_age}?, yeah dont expect to be older than him >:)");
                println!("Im ending the game for now, come back when you become older, >:)");
                break;
            },
            Ordering::Greater => {
                println!("Wait... your actually older than him, HOW ARE YOU {users_age} YEARS OLD????");
                println!("Your too old for this, im ending the game, bye :)");
                break;
            },
            Ordering::Equal => {
                println!("Wow.. your the same age as him, how are you even alive??");
                println!("Since you guessed his age I guess the game is over, bye!");
                break;
            }
        };
      
    }






}
