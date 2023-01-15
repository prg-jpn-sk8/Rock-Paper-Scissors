#![allow(unused_variables)]
use rand::thread_rng;
use rand::seq::SliceRandom;
use std::io::stdin;

fn main() {
    let mut x = 0;
    let mut y = 0;
    let list = vec!["paper", "scissors", "rock"];
    loop {
        let bot = list.choose(&mut thread_rng()).unwrap();
        let pc = bot.trim();
        println!("Play rock/paper/scissors with me : ");
        let mut input = String::new();
        stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
        let player: &str = input.trim();

        if pc == player {
            println!("Tie , Player score = {} and Pc score = {} ", x, y);
            println!("========");
        }
        else if player == "paper" && pc == "rock" || player == "scissors" && pc == "paper" || player == "rock" && pc == "scissors"{
            x += 1;
            println!("You win, Player score = {} and Pc score = {} ", x, y);
            println!("========");
        } else if pc == "paper" && player == "rock" || pc == "scissors" && player == "paper" || pc == "rock" && player == "scissors" {
            y += 1;
            println!("You lose , Player score = {} and Pc score = {} ", x, y);
            println!("========");
        } else {
            println!("Error");
        }
    }
}