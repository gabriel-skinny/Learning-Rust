use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!!"); 
    
    let secret_number = rand::thread_rng().gen_range(1..101);

    let quit = String::from("quit");
        
    loop{
        
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("You guessed {}", guess);
        
        match guess.cmp(&quit) {
            Ordering::Equal => {
                println!("Quiting...");
                break;
            }
            Ordering::Less => print!("no"),
            Ordering::Greater => print!("no"),
        }


        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("To small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }

        println!("The random number is {}", secret_number); 
    }
}
