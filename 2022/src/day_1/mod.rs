#[allow(dead_code)]
pub fn sum_1_calorie(src: &str) -> usize {
    calorie_counting(src)[0]
}

pub fn sum_2_calorie(src: &str) -> usize {
    let vec = calorie_counting(src);

    vec[0] + vec[1] + vec[2]
}

#[allow(dead_code)]
fn calorie_counting(src: &str) -> Vec<usize> {
    let mut vec: Vec<usize> = src
        .split("\n\r")
        .map(|v| v.trim())
        .map(|v| -> usize {
            v.split("\r\n")
                .map(|v| -> usize { v.parse().unwrap_or(0) })
                .sum()
        })
        .collect::<Vec<usize>>();

    vec.sort_by(|a, b| b.cmp(a));
    vec
}

#[allow(unused_imports)]
mod test {
    use super::{sum_1_calorie, sum_2_calorie};

    #[test]
    fn case_1() {
        let data = "1000\r\n2000\r\n3000\r\n\r\n4000\r\n\r\n5000\r\n6000\r\n\r\n7000\r\n8000\r\n9000\r\n\r\n10000";
        let result = sum_1_calorie(&data);
        assert_eq!(result, 24000);
    }

    #[test]
    fn case_2() {
        let data = "1000\r\n2000\r\n3000\r\n\r\n4000\r\n\r\n5000\r\n6000\r\n\r\n7000\r\n8000\r\n9000\r\n\r\n10000";
        let result = sum_2_calorie(&data);
        assert_eq!(result, 45000);
    }
}
