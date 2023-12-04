use std::{fs::File, io::Read};

use day_1::day_2::{power_sum_of_games, sum_of_games};

fn main() {
    let mut file = File::open("src/day_2/input.txt").expect("[-] Expected file input.txt");
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)
        .expect("[-] Failed to get a string");

    println!("{}", power_sum_of_games(&buffer));
}
