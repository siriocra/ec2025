use regex::Regex;

pub fn part1(input: String) -> i64 {
    let numbers = input.split_whitespace().map(|x| x.parse::<i64>().unwrap()).collect::<Vec<_>>();
    let mut dial = vec![1];
    for i in (0..numbers.len()).step_by(2) {
        dial.push(numbers[i]);
    }
    for i in (1..numbers.len()).step_by(2).rev() {
        dial.push(numbers[i]);
    }
    return dial[2025 % dial.len()];
}

fn parse_input(input: String) -> Vec<(i64, i64)>{
    let mut ranges = Vec::new();
    for line in input.lines() {    
        let re = Regex::new(r"(\d+)-(\d+)$").unwrap();
        let captures = re.captures(&line).unwrap();
        ranges.push((
            captures[1].parse::<i64>().unwrap(),
            captures[2].parse::<i64>().unwrap(),
        ));
    }
    return ranges;
}

pub fn part2(input: String) -> i64 {
    let ranges = parse_input(input);
    let mut dial = vec![1];
    for i in (0..ranges.len()).step_by(2) {
        for j in ranges[i].0..=ranges[i].1 {
            dial.push(j);
        }
    }
    for i in (1..ranges.len()).step_by(2).rev() {
        for j in (ranges[i].0..=ranges[i].1).rev() {
            dial.push(j);
        }
    }
    return dial[20252025 % dial.len()];
}

fn range_count(range: (i64, i64)) -> i64 {
    return (range.0 - range.1).abs() + 1;
}

pub fn part3(input: String) -> i64 {
    let ranges = parse_input(input);
    let mut dial = vec![(1, 1)];
    let mut count = 1;
    for i in (0..ranges.len()).step_by(2) {
        dial.push(ranges[i]);
        count += ranges[i].1 - ranges[i].0 + 1;
    }
    for i in (1..ranges.len()).step_by(2).rev() {
        dial.push((ranges[i].1, ranges[i].0));
        count += ranges[i].1 - ranges[i].0 + 1;
    }
    let mut modulo = 202520252025 % count;
    let mut cur_dial = 0;
    while modulo >= range_count(dial[cur_dial]) {
        modulo -= range_count(dial[cur_dial]);
        cur_dial += 1;
    }
    let sign = if dial[cur_dial].0 < dial[cur_dial].1 {1} else {-1};
    return dial[cur_dial].0 + modulo * sign;
}

mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(String::from("72
58
47
61
67")), 67);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(String::from("10-15
12-13
20-21
19-23
30-37")), 30);
    }

    #[test]
    fn test_part3() {
        assert_eq!(part3(String::from("10-15
12-13
20-21
19-23
30-37")), 30);
    }
}