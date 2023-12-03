use std::{fs::File, io::Read};

fn match_calibration_part_1(line: &str) -> u32 {
    let mut it = line.chars().filter_map(|character| character.to_digit(10));

    let first = it.next().unwrap();
    let last = it.last();

    match last {
        Some(last) => format!("{first}{last}"),
        None => format!("{first}{first}"),
    }
    .parse::<u32>()
    .unwrap()
}

fn match_calibration_part_2(line: &str) -> u32 {
    let mut it = (0..line.len()).filter_map(|index| {
        let reduced = &line[index..];

        let result = if reduced.starts_with("one") {
            '1'
        } else if reduced.starts_with("two") {
            '2'
        } else if reduced.starts_with("three") {
            '3'
        } else if reduced.starts_with("four") {
            '4'
        } else if reduced.starts_with("five") {
            '5'
        } else if reduced.starts_with("six") {
            '6'
        } else if reduced.starts_with("seven") {
            '7'
        } else if reduced.starts_with("eight") {
            '8'
        } else if reduced.starts_with("nine") {
            '9'
        } else {
            reduced.chars().next().unwrap()
        };

        result.to_digit(10)
    });

    let first = it.next().unwrap();
    let last = it.last();

    match last {
        Some(last) => format!("{first}{last}"),
        None => format!("{first}{first}"),
    }
    .parse::<u32>()
    .unwrap()
}

// fn main() {
//     let mut file = File::open("input.txt").expect("[-] Expected file input.txt");
//     let mut buffer = String::new();
//     file.read_to_string(&mut buffer)
//         .expect("[-] Failed to get a string");
//
//     let sum = buffer.lines().map(match_calibration_part_2).sum::<u32>();
//
//     println!("Result : {sum}");
// }
