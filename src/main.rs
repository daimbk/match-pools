// script to generate match pools
use std::io::{self, Write};


fn get_player_names(total_players: i32) -> Vec<String> {
    let mut player_names: Vec<String> = Vec::new();
    
    for _ in 0..total_players {
        print!("Enter player name: ");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let player_name: String = input.trim().to_string();
        player_names.push(player_name);
    }

    return player_names;
}

fn main() {
    print!("Total number of players: ");
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let total_players: i32 = input.trim().parse().expect("Invalid input");

    // calculate total number of unique matches
    let total_matches: i32 = total_players * (total_players - 1) / 2;

    println!("Total number of matches: {}", total_matches);

    // get player names in a list
    let player_names: Vec<String> = get_player_names(total_players);
}
