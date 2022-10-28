//Guessing Game


use rand::{thread_rng, Rng};
use std::io;
use std::cmp::Ordering;

fn main() {
    println!("Welcome to the Game");
    let secret_num = rand::thread_rng().gen_range(1..101);
    //println!("Secret number is : {}",secret_num);
    loop{
    println!("input your guess number");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess)
        .expect("Failed to read the line");
    println!("Your Guess:{}",guess);
    let guess:u32 = guess.trim().parse().expect("Type an integar"); //converting to integar

    match guess.cmp(&secret_num){
        Ordering:: Less => println!("Lesser bro"),
        Ordering:: Greater => println!("Too big bro num"),
        Ordering:: Equal => {
            println!("Dammnn bro you win");
            break;
            }
        }   
    }

}
