use std::collections::HashMap;

pub fn part1(input: String) -> i32 {
    let mut mentors: HashMap<char, i32> = HashMap::new();
    let mut ans = 0;
    for c in input.as_str().chars() {
        if c.is_ascii_uppercase() {
            *mentors.entry(c).or_insert(0) += 1;
        } else {
            // Novice
            let m = *mentors.entry(c.to_ascii_uppercase()).or_insert(0);
            if c == 'a' {
                ans += m;
            }
        }
    }
    return ans;
}

pub fn part2(input: String) -> i32 {
    let mut mentors: HashMap<char, i32> = HashMap::new();
    let mut ans = 0;
    for c in input.as_str().chars() {
        if c.is_ascii_uppercase() {
            *mentors.entry(c).or_insert(0) += 1;
        } else {
            // Novice
            let m = *mentors.entry(c.to_ascii_uppercase()).or_insert(0);
            ans += m;
        }
    }
    return ans;
}

fn calc_start(input: &Vec<char>, left: i32, right: i32, distance: i32) -> i32 {
    let mut mentors: HashMap<char, i32> = HashMap::new();
    let mut ans = 0;
    for i in left..=right {
        let c = input[i as usize];
        if c.is_ascii_uppercase() {
            *mentors.entry(c).or_insert(0) += 1;
        }
    }
    for i in left..right {
        let c = input[i as usize];
        if c.is_ascii_lowercase() {
            // Novice
            let m = *mentors.entry(c.to_ascii_uppercase()).or_insert(0);
            ans += m;
        }
        let r = input[(i + distance + 1) as usize];
        if r.is_ascii_uppercase() {
            *mentors.entry(r).or_insert(0) += 1;
        }
    }
    return ans;
}

fn calc_end(input: &Vec<char>, left: i32, right: i32, distance: i32) -> i32 {
    let mut mentors: HashMap<char, i32> = HashMap::new();
    let mut ans = 0;
    for i in left-distance..right {
        let c = input[i as usize];
        if c.is_ascii_uppercase() {
            *mentors.entry(c).or_insert(0) += 1;
        }
    }
    for i in left..right {
        let c = input[i as usize];
        if c.is_ascii_lowercase() {
            // Novice
            let m = *mentors.entry(c.to_ascii_uppercase()).or_insert(0);
            ans += m;
        }
        let l = input[(i - distance) as usize];
        if l.is_ascii_uppercase() {
            *mentors.entry(l).or_insert(0) -= 1;
        }
    }
    return ans;
}

fn calc_mentors(input: String, distance: i32, repetitions: i32) -> i32 {
    let mut mentors: HashMap<char, i32> = HashMap::new();
    let large_str = input.clone() + input.as_str() + input.as_str();
    let chars = large_str.chars().collect::<Vec<_>>();
    let length = input.as_str().len() as i32;
    let mut ans = calc_start(&chars, 0, distance, distance);
    for i in length-distance..=length+distance {
        let c = chars[i as usize];
        if c.is_ascii_uppercase() {
            *mentors.entry(c).or_insert(0) += 1;
        }
    }
    for i in length..length*2 {
        let c = chars[i as usize];
        if c.is_ascii_lowercase() {
            let m = *mentors.entry(c.to_ascii_uppercase()).or_insert(0);
            if (i - length) < distance || i + distance >= length * 2 {
                ans += m * (repetitions - 1);
            } else {
                ans += m * repetitions;
            }
        }
        let left = chars[(i - distance) as usize];
        let right = chars[(i + distance + 1) as usize];
        if left.is_ascii_uppercase() {
            *mentors.entry(left).or_insert(0) -= 1;
        }
        if right.is_ascii_uppercase() {
            *mentors.entry(right).or_insert(0) += 1;
        }
    }
    return ans + calc_end(&chars, length*2 - distance, length*2, distance);
}

pub fn part3(input: String) -> i32 {
    return calc_mentors(input, 1000, 1000);
}

mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(String::from("ABabACacBCbca")), 5);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(String::from("ABabACacBCbca")), 11);
    }

    #[test]
    fn test_calc_mentors() {
        assert_eq!(calc_mentors(String::from("AABCBABCABCabcabcABCCBAACBCa"), 10, 1), 34);
        assert_eq!(calc_mentors(String::from("AABCBABCABCabcabcABCCBAACBCa"), 10, 2), 72);
    }

    #[test]
    fn test_part3() {
        let str = String::from("AABCBABCABCabcabcABCCBAACBCa");
        let mut long_str = String::new();
        for _ in 0..1000 {
            long_str += str.as_str();
        }
        assert_eq!(calc_mentors(long_str, 1000, 1), 3442321);
    }
}