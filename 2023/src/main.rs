use std::{fs::File, io::Read};

use day_1::day_2::sum_of_games;

fn main() {
    //     let src = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    // Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    // Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    // Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    // Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    let mut file = File::open("src/day_2/input.txt").expect("[-] Expected file input.txt");
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)
        .expect("[-] Failed to get a string");

    println!("{}", sum_of_games(&buffer));
}
