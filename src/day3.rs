use std::cmp::max;

pub fn part1(input:String) -> i32 {
    let numbers_str = input.split(",").collect::<Vec<_>>();
    let mut numbers = numbers_str.into_iter().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>();
    numbers.sort_by(|a, b| b.cmp(a));

    let mut ans = numbers[0];
    for i in 1..numbers.len() {
        if numbers[i] != numbers[i-1] {
            ans += numbers[i];
        }
    }
    return ans;
}

pub fn part2(input:String) -> i32 {
    let numbers_str = input.split(",").collect::<Vec<_>>();
    let mut numbers = numbers_str.into_iter().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>();
    numbers.sort();

    let mut ans = numbers[0];
    let mut crates = 1;
    for i in 1..numbers.len() {
        if numbers[i] != numbers[i-1] {
            ans += numbers[i];
            crates += 1;
            if crates == 20 {
                break;
            }
        }
    }
    return ans;
}

pub fn part3(input:String) -> i32 {
    let numbers_str = input.split(",").collect::<Vec<_>>();
    let mut numbers = numbers_str.into_iter().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>();
    numbers.sort();
    
    let mut sets = 1;
    let mut ans = 1;
    for i in 1..numbers.len() {
        if numbers[i] != numbers[i-1] {
            sets = 1;
        } else {
            sets += 1;
            ans = max(ans, sets);
        }
    }
    return ans;
}

mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(String::from("10,5,1,10,3,8,5,2,2")), 29);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(
            String::from("4,51,13,64,57,51,82,57,16,88,89,48,32,49,49,2,84,65,49,43,9,13,2,3,75,72,63,48,61,14,40,77")), 
            781,
        );
    }

    #[test]
    fn test_part3() {
        assert_eq!(part3(
            String::from("4,51,13,64,57,51,82,57,16,88,89,48,32,49,49,2,84,65,49,43,9,13,2,3,75,72,63,48,61,14,40,77")),
            3
        )
    }
}