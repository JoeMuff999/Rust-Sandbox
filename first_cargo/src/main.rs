use rand::Rng;
use std::io;
use std::cmp::Ordering;

fn main() {

    let max_value: u32 = 100;
    let rand_value = rand::thread_rng().gen_range(1, max_value);

    loop {
        println!("make a guess!");
        let mut guess = String::new();
        
        io::stdin()
            .read_line(&mut guess)
            .expect("Please input a guess :)"); //read into the guess reference
        //shadowing with guess, parses as u32 given the string guess. this way, we don't need two separate variables
        let guess = match guess.trim().parse::<u32>() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&rand_value)
        {
            Ordering::Less => println!("Too low"),
            Ordering::Greater => println!("Too High"),
            Ordering::Equal => {
                println!("Correct, the answer was {0}!", guess);
                break;
            }

        }
    }
}
