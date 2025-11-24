use core::num;

fn calc_checksum(numbers: &Vec<i64>) -> i64 {
    let mut checksum: i64 = 0;
    for i in 0..numbers.len() {
        checksum += (i as i64 + 1) * numbers[i];
    }
    return checksum;
}

fn one_round(numbers: &mut Vec<i64>, stage_1: bool) {
    if stage_1 {
        for i in 0..numbers.len()-1 {
            if numbers[i] > numbers[i+1] {
                numbers[i] -= 1;
                numbers[i+1] += 1;
            }
        }
        
    } else {
        for i in 0..numbers.len()-1 {
            if numbers[i] < numbers[i+1] {
                numbers[i] += 1;
                numbers[i+1] -= 1;
            }
        }
    }
}

pub fn part1(input: String) -> i64 {
    let mut numbers = input.split_whitespace().map(|x| x.parse::<i64>().unwrap()).collect::<Vec<_>>();
    let mut stage_1 = true;
    'round: for round in 1..10 {
        one_round(&mut numbers, stage_1);
        if stage_1 {
            for i in 0..numbers.len()-1 {
                if numbers[i] > numbers[i+1] {
                    continue 'round;
                }
            }
        }
        stage_1 = false;
    }
    return calc_checksum(&numbers);
}

fn calc_average(numbers: &Vec<i64>) -> i64 {
    let mut ans: i64 = 0;
    for i in 0..numbers.len() {
        ans += numbers[i];
    }
    return ans / numbers.len() as i64;
}

pub fn part2(input: String) -> i64 {
    let mut numbers = input.split_whitespace().map(|x| x.parse::<i64>().unwrap()).collect::<Vec<_>>();
    let average = calc_average(&numbers);
    let mut stage_1 = true;
    let mut round = 0;
    'round: loop {
        one_round(&mut numbers, stage_1);
        round += 1;
        if stage_1 {
            for i in 0..numbers.len()-1 {
                if numbers[i] > numbers[i+1] {
                    continue 'round;
                }
            }
        }
        stage_1 = false;
        for i in 0..numbers.len()-1 {
            if numbers[i] != average {
                continue 'round;
            }
        }
        break;
    }
    return round;
}

pub fn part3(input: String) -> i64 {
    let mut numbers = input.split_whitespace().map(|x| x.parse::<i64>().unwrap()).collect::<Vec<_>>();
    let average = calc_average(&numbers);
    let mut sum: i64 = 0;
    for i in 0..numbers.len() {
        if numbers[i] < average {
            sum += average - numbers[i];
        }
    }
    return sum;
}

mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(String::from("9
1
1
4
9
6")), 109);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(String::from("9
1
1
4
9
6")), 11);
        assert_eq!(part2(String::from("805
706
179
48
158
150
232
885
598
524
423")), 1579);
    }

    #[test]
    fn test_part3() {
        assert_eq!(part3(String::from("1
2
3
4
5")), 3);
        assert_eq!(part3(String::from("3
3
3
3
3
9")), 5);
    }
}