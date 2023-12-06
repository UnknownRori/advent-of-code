const SYMBOL: [char; 4] = ['*', '$', '+', '#'];

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
        if !char.is_digit(10) && !temp_str.is_empty() {
            temp.push(SlicesPart {
                slice: temp_str.to_owned(),
                start: usize::try_from(start).unwrap(),
                end: i,
            });

            temp_str = String::new();
            start = -1;
            continue;
        } else if !char.is_digit(10) && temp_str.is_empty() {
            continue;
        }

        if start < 0 {
            start = i32::try_from(i).unwrap();
        }

        temp_str.push(char);
    }

    temp
}

// INFO : It's stupid but it work, btw Refactor later
#[allow(dead_code)]
pub fn sum_part_number(src: &str) -> u32 {
    let lines: Vec<&str> = src.lines().collect();
    let mut visited: Vec<(usize, usize)> = vec![];
    let mut sum = 0;

    for y in 0..lines.len() {
        let line = lines.get(y).unwrap();
        for x in 0..line.len() {
            let char = line.chars().nth(x).unwrap();

            if SYMBOL.contains(&char) {
                if let Some(top_line) = lines.get(y - 1) {
                    let slices = get_slices_part(top_line);
                    for slice in slices {
                        if ((slice.start > x - 1 && slice.start <= x + 1)
                            || (slice.end > x - 1 && slice.end <= x + 1))
                            && !visited.contains(&(slice.start, y))
                        {
                            visited.push((slice.start, y - 1));
                            sum += slice.slice.parse::<u32>().unwrap();
                        }
                    }
                }

                if let Some(bottom_line) = lines.get(y + 1) {
                    let slices = get_slices_part(bottom_line);
                    for slice in slices {
                        if ((slice.start > x - 1 && slice.start <= x + 1)
                            || (slice.end > x - 1 && slice.end <= x + 1))
                            && !visited.contains(&(slice.start, y))
                        {
                            visited.push((slice.start, y + 1));
                            sum += slice.slice.parse::<u32>().unwrap();
                        }
                    }
                }

                if let Some(right_char) = line.chars().nth(x + 1) {
                    if right_char.is_numeric() && !visited.contains(&(x + 1, y)) {
                        let mut temp = String::new();
                        let right = line.chars().skip(x + 1);

                        for char in right {
                            if !char.is_numeric() {
                                break;
                            }

                            temp.push(char);
                        }

                        visited.push((x + 1, y));
                        sum += temp.parse::<u32>().unwrap();
                    }
                }

                if let Some(left_char) = line.chars().nth(x - 1) {
                    if left_char.is_numeric() && !visited.contains(&(x - 1, y)) {
                        let mut temp = String::new();
                        let left = line.chars().rev().skip(line.len() - x);

                        for char in left {
                            if !char.is_numeric() {
                                visited.push((x - 1, y));
                                break;
                            }
                            temp.push(char);
                        }

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

    sum
}
