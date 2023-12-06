use std::{fs::File, io::Read};

use day_1::day_3::sum_part_number;

fn main() {
    let mut file = File::open("src/day_3/input.txt").expect("[-] Expected file input.txt");
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)
        .expect("[-] Failed to get a string");

    //     let buffer = "467..114..
    // ...*......
    // ..35..633.
    // ......#...
    // 617*......
    // .....+.58.
    // ..592.....
    // ......755.
    // ...$.*....
    // .664.598.."; //62
    //     println!("{buffer}\n");

    println!("{}", sum_part_number(&buffer));
}
