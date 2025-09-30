use colored::*;

use rand::Rng;
use std::cmp::Ordering;
use std::{io, u32}; // Y: Previously used only u32 but got suggestoins.

fn main() {
    print_values();
    print_deatis();
    run_the_game();
}

fn print_deatis() {
    println!("I have came Rust !!!");
    println!("WHAT I LEARN TILL NOW : ");
    println!("install && update carggo and rustup ");
    println!("open local docs in ~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/share/doc/rust/html/*");
    println!("build & run program using cargo ");
    println!("cargo init = inintialise rust project");
    println!("cargo build = will build a binary ./target/debug/filename (debug mode)");
    println!("cargo run = will run the binary ");
    println!("cargo check = check for errors in file");
    println!("cargo bulid --release = build as release mode ./target/release/filename");
    println!("cargo run = build & run in single command");
    println!("cargo run --release = build & run as release mode ./target/release/filename");
    println!("cargo -watch (tailwind like --watch command )");
}

fn print_values() {
    //R: printing values;
    let x = 4;
    let y = 12;
    let ghonchu = 422;
    println!("{}", ghonchu);
    println!(
        "here, X = {x}, Y + 3 = {}, and newly created Z = {}",
        y + 3,
        50
    );
}

fn extra_instructions() {
    println!("next functionallitye is to add :
========================================================================================================
1. everyt time uer inout we are allready checking if it big or small , now we have to check how close the users guess is.
2. we will be needing 2 different mutable variables, a. for closest smaller guess and b. closest biger guess.
3. each time the guess is compaired with the exciting one guess if the guess is closer the newer will replace the older one.
4. and with the output we will be showing the result to choose in the newer 'shringked' range  with the help of a and b.
5. with this method we can increase the guess range drastically and add levels like easy, modaret, hard, and assested game or unassisted.
")
}

fn run_the_game() {
    println!(
        "|---------------------------------| Guessing Game |---------------------------------|"
    );

    let randome_number = rand::thread_rng().gen_range(1, 1001);

    loop {
        println!("😎 Please input your Guess in the range of 1 to 1000:");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Falure to read the line. !!!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("🚫 Please Input Numbers Only !");
                continue;
            }
        };

        match guess.cmp(&randome_number) {
            Ordering::Less => {
                let less_number = randome_number - guess;
                println!(
                    "Too Small ⏬ \n Your number is {} smaller than actuall number.",
                    stringify!(less_number).red()
                )
            }

            Ordering::Greater => println!("{}", "Too Big ⏫".red()),
            Ordering::Equal => {
                println!("{}", "You Won !!! 💛💙🩶🩷🤍💖💛💙🩶🩷🤍💖".green());
                extra_instructions();
                break;
            }
        }
    }
}
