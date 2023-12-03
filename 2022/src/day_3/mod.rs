pub fn sum_priority_items(src: &str) -> usize {
    src.split("\n")
        .map(|v| v.trim())
        .map(|v| -> usize {
            let (l, r) = v.split_at(v.len() / 2);

            let mut vec = vec![];
            r.chars()
                .map(|v| {
                    if !(vec.contains(&v)) && l.contains(v) {
                        vec.push(v);
                        return calculate_priorty(v);
                    }
                    0
                })
                .sum()
        })
        .sum()
}

#[inline]
fn calculate_priorty(target: char) -> usize {
    if target.is_lowercase() {
        return target as usize - 'a' as usize + 1;
    }

    target as usize - 'A' as usize + 27
}

pub fn sum_badges(src: &str) -> usize {
    let src: Vec<&str> = src.split("\n").map(|v| v.trim()).collect();

    src.chunks(3)
        .map(|v| -> usize {
            let mut vec = vec![];

            for i in v[0].chars() {
                vec.push(i);
            }

            for i in v[1].chars() {
                if vec.contains(&i) {
                    for j in v[2].chars() {
                        if i == j {
                            return calculate_priorty(j);
                        }
                    }
                }
            }
            0
        })
        .sum()
}

#[allow(unused_imports)]
mod test {
    use super::{sum_badges, sum_priority_items};

    #[test]
    fn case_1() {
        let data = "vJrwpWtwJgWrhcsFMMfFFhFp
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
        PmmdzqPrVvPwwTWBwg
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
        ttgJtRGJQctTZtZT
        CrZsJsPPZsGzwwsLwLmpwMDw";
        assert_eq!(sum_priority_items(data), 157);
    }

    #[test]
    fn case_2() {
        let data = "vJrwpWtwJgWrhcsFMMfFFhFp
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
        PmmdzqPrVvPwwTWBwg
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
        ttgJtRGJQctTZtZT
        CrZsJsPPZsGzwwsLwLmpwMDw";
        assert_eq!(sum_badges(data), 70);
    }
}
