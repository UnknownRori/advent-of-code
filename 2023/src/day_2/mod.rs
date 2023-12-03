const MAX_RED: i32 = 12;
const MAX_GREEN: i32 = 13;
const MAX_BLUE: i32 = 14;

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
