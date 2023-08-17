// script to generate match pools
use std::io;


fn main() {
    println!("Total number of players: ");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let total_players: i32 = input.trim().parse().expect("Invalid input");

    // calculate total number of unique matches
    let total_matches: i32 = total_players * (total_players - 1) / 2;

    println!("Total number of matches: {}", total_matches);
}
