//Creating a guessing game from doc.rust-lang.org/book Chapter 2: Programming a Guessing Game.
//Practiced by Jonathan Brown
//jonathan.z.r.brown@gmail.com
//2/19/2022
use std::io; //inout/output library from the std library.
//if you want to use something without this prelude, you must use library reference.
//ex: std::io::stdin()
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

//DEBUG LINE    println!("The secret number is: {}", secret_number);

    loop{
        println!("Please input your guess.");

        //Variable for holding user input.
        //new creates a new empty string.
        let mut guess = String::new();
        

        //returns an instance of std::io::stdin
        //represents a handle to standard input from terminal.
        io::stdin()
            //this actually gets input from user.
            //&mut guess passes where to store the data (append).
            //& is a reference. avoids copying data, but shares the memory location referenced.
            .read_line(&mut guess)
            .expect("Failed to read line");
            //equivilant to io::stdin().read_line(&mut guess).expect("Failed to read line");
            //Result's enum variants are OK or Err.
            //expect calls a result method. If err, display message passed in argument.

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small..."),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }

}

