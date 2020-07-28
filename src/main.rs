extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
	let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("Guess number!");
	println!("Please define your hypothesis.");

	loop {
		let mut guess = String::new();
	
		io::stdin().read_line(&mut guess)
			.ok()
			.expect("Something wrong");

		let guess_number: u32 = match guess.trim().parse() {
			Ok(num) => num,
			Err(_) => {
				continue;
			},
		};

		println!("Your attempt is {}", guess_number);
		
		match guess_number.cmp(&secret_number) {
			Ordering::Less    => println!("Too small"),
			Ordering::Greater => println!("Too big"),
			Ordering::Equal   => {
				println!("You win!");
				break;
			},
		}
	}
}
