#[derive(Debug)]
struct SlicesPart {
    slice: String,
    start: usize,
    end: usize,
}

// INFO : It's stupid but it work
fn get_slices_part(line: &str) -> Vec<SlicesPart> {
    let mut temp = vec![];

    let mut temp_str = String::new();
    let mut start: i32 = -1;
    for (i, char) in line.chars().enumerate() {
        if !char.is_numeric() && !temp_str.is_empty() {
            temp.push(SlicesPart {
                slice: temp_str.to_owned(),
                start: usize::try_from(start).unwrap(),
                end: i - 1,
            });

            temp_str = String::new();
            start = -1;
            continue;
        } else if !char.is_numeric() && temp_str.is_empty() {
            continue;
        }

        if start < 0 {
            start = i32::try_from(i).unwrap();
        }

        temp_str.push(char);
    }

    if !temp_str.is_empty() {
        temp.push(SlicesPart {
            slice: temp_str.to_owned(),
            start: usize::try_from(start).unwrap(),
            end: line.chars().count() - 1,
        });
    }

    temp
}

// INFO : It's stupid but it work, btw Refactor later
#[allow(dead_code)]
pub fn sum_part_number(src: &str) -> u32 {
    let lines: Vec<&str> = src.lines().collect();
    let mut visited: Vec<(usize, usize, u32)> = vec![];
    let mut sum = 0;

    for y in 0..lines.len() {
        let line = lines.get(y).unwrap();
        for x in 0..line.len() {
            let char = line.chars().nth(x).unwrap();

            if !char.is_numeric() && char != '.' {
                if y > 0 {
                    if let Some(top_line) = lines.get(y - 1) {
                        let slices = get_slices_part(top_line);
                        for slice in slices {
                            if !visited.contains(&(
                                slice.start,
                                y - 1,
                                slice.slice.parse().unwrap(),
                            )) {
                                if x == 0 {
                                    if (slice.start >= x && slice.start <= x + 1)
                                        || (slice.end >= x - 1 && slice.end <= x + 1)
                                    {
                                        visited.push((
                                            slice.start,
                                            y,
                                            slice.slice.parse().unwrap(),
                                        ));
                                        sum += slice.slice.parse::<u32>().unwrap();
                                    }
                                } else {
                                    if (slice.start >= x - 1 && slice.start <= x + 1)
                                        || (slice.end >= x - 1 && slice.end <= x + 1)
                                    {
                                        visited.push((
                                            slice.start,
                                            y - 1,
                                            slice.slice.parse().unwrap(),
                                        ));
                                        sum += slice.slice.parse::<u32>().unwrap();
                                    }
                                }
                            }
                        }
                    }
                }

                if y < lines.len() {
                    if let Some(bottom_line) = lines.get(y + 1) {
                        let slices = get_slices_part(bottom_line);
                        for slice in slices {
                            if !visited.contains(&(
                                slice.start,
                                y + 1,
                                slice.slice.parse().unwrap(),
                            )) {
                                if x == 0 {
                                    if (slice.start >= x && slice.start <= x + 1)
                                        || (slice.end >= x - 1 && slice.end <= x + 1)
                                    {
                                        visited.push((
                                            slice.start,
                                            y,
                                            slice.slice.parse().unwrap(),
                                        ));
                                        sum += slice.slice.parse::<u32>().unwrap();
                                    }
                                } else {
                                    // TODO : Somewhere around here are subtract overflow
                                    if (slice.start >= x - 1 && slice.start <= x + 1)
                                        || (slice.end >= x - 1 && slice.end <= x + 1)
                                    {
                                        visited.push((
                                            slice.start,
                                            y + 1,
                                            slice.slice.parse().unwrap(),
                                        ));
                                        sum += slice.slice.parse::<u32>().unwrap();
                                    }
                                }
                            }
                        }
                    }
                }

                if let Some(right_char) = line.chars().nth(x + 1) {
                    if right_char.is_numeric() {
                        let mut temp = String::new();
                        let right = line.chars().skip(x + 1);

                        for char in right {
                            if !char.is_numeric() {
                                break;
                            }

                            temp.push(char);
                        }

                        if !visited.contains(&(x + 1, y, temp.parse::<u32>().unwrap())) {
                            visited.push((x + 1, y, temp.parse::<u32>().unwrap()));
                            sum += temp.parse::<u32>().unwrap();
                        }
                    }
                }

                let left_half = if x > 0 {
                    line.chars().nth(x - 1)
                } else {
                    line.chars().nth(x)
                };
                if let Some(left_char) = left_half {
                    if left_char.is_numeric() {
                        let mut temp = String::new();
                        let left = line.chars().rev().skip(line.len() - x);

                        for char in left {
                            if !char.is_numeric() {
                                break;
                            }
                            temp.push(char);
                        }

                        if !visited.contains(&(
                            x - temp.len(),
                            y,
                            temp.chars()
                                .rev()
                                .collect::<String>()
                                .parse::<u32>()
                                .unwrap(),
                        )) {
                            visited.push((
                                x - temp.len(),
                                y,
                                temp.chars()
                                    .rev()
                                    .collect::<String>()
                                    .parse::<u32>()
                                    .unwrap(),
                            ));
                            sum += temp
                                .chars()
                                .rev()
                                .collect::<String>()
                                .parse::<u32>()
                                .unwrap();
                        }
                    }
                }
            }
        }
    }
    sum
}

#[cfg(test)]
mod test {
    use crate::day_3::sum_part_number;

    #[test]
    fn test_case_1() {
        let buffer = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let result = sum_part_number(buffer);
        assert_eq!(result, 4361);
    }

    #[test]
    fn test_case_reddit_1() {
        let buffer = "12.......*..
+.........34
.......-12..
..78........
..*....60...
78.........9
.5.....23..$
8...90*12...
............
2.2......12.
.*.........*
1.1..503+.56";
        let result = sum_part_number(buffer);
        assert_eq!(result, 925);
    }

    #[test]
    fn test_case_reddit_2() {
        let buffer = ".......5......
..7*..*.....4*
...*13*......9
.......15.....
..............
..............
..............
..............
..............
..............
21............
...*9.........";

        let result = sum_part_number(buffer);
        assert_eq!(result, 62);
    }

    #[test]
    fn test_case_reddit_3() {
        let buffer = ".......5......
..7*..*.......
...*13*.......
.......15.....";

        let result = sum_part_number(buffer);
        assert_eq!(result, 40);
    }
}
