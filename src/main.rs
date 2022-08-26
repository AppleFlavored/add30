use std::io;

use rand::{rngs::ThreadRng, Rng};

fn guess(random: &mut ThreadRng, t: i32) -> i32 {
    if t >= 25 {
        30 - t
    } else {
        let r = random.gen_range(1..=5);
        r.min(30 - t)
    }
}

fn main() {
    let mut total = 0;
    let mut random = rand::thread_rng();

    loop {
        println!("Pick a number between 1 and 5:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let number = input.trim().parse::<i32>().unwrap();
        if !(1..=5).contains(&number) {
            continue;
        }

        total += number;
        println!("You chose {}. The total is now {}.", number, total);
        if total >= 30 {
            println!("----------\nYou won!\n----------");
            break;
        }

        let comp_num = guess(&mut random, total);
        total += comp_num;
        println!("The computer chose {}. The total is now {}.", comp_num, total);
        if total >= 30 {
            println!("----------\nThe computer won!\n----------");
            break;
        }
    }
}
