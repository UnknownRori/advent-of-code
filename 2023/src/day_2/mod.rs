const MAX_RED: i32 = 12;
const MAX_GREEN: i32 = 13;
const MAX_BLUE: i32 = 14;

#[allow(dead_code)]
pub fn power_sum_of_games(src: &str) -> i32 {
    src.lines()
        .map(|line| -> i32 {
            let mut split = line.split(": ");

            let mut game_id = split.next().unwrap().split(" ");
            let _ = game_id.next();

            let games_record = split.next().unwrap();
            let games = games_record.split(";");

            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;

            for game in games {
                let game = game.trim().split(", ");
                for game in game {
                    let mut split = game.split(" ");
                    let num = split.next().unwrap().parse::<i32>().unwrap();
                    let game_type = split.next().unwrap();

                    match game_type {
                        "red" => red = red.max(num),
                        "blue" => blue = blue.max(num),
                        "green" => green = green.max(num),
                        _ => {}
                    }
                }
            }

            red * green * blue
        })
        .sum()
}

#[allow(dead_code)]
pub fn sum_of_games(src: &str) -> i32 {
    src.lines()
        .map(|line| -> i32 {
            let mut split = line.split(": ");

            let mut game_id = split.next().unwrap().split(" ");
            let _ = game_id.next();

            let games_record = split.next().unwrap();
            let games = games_record.split(";");

            for game in games {
                let game = game.trim().split(", ");
                for game in game {
                    let mut split = game.split(" ");
                    let num = split.next().unwrap().parse::<i32>().unwrap();
                    let game_type = split.next().unwrap();

                    match game_type {
                        "red" => {
                            if num > MAX_RED {
                                return 0;
                            }
                        }
                        "blue" => {
                            if num > MAX_BLUE {
                                return 0;
                            }
                        }
                        "green" => {
                            if num > MAX_GREEN {
                                return 0;
                            }
                        }
                        _ => {}
                    }
                }
            }

            let id = game_id.next().unwrap().parse::<i32>().unwrap();
            id
        })
        .sum()
}

// fn main() {
//     let mut file = File::open("src/day_2/input.txt").expect("[-] Expected file input.txt");
//     let mut buffer = String::new();
//     file.read_to_string(&mut buffer)
//         .expect("[-] Failed to get a string");
//
//     println!("{}", power_sum_of_games(&buffer));
// }
