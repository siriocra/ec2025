use regex::Regex;

fn parse_input(input:String) -> Vec<i32> {
    input.lines().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>()
}

pub fn part1(input:String) -> i32 {
    let gears = parse_input(input);
    let first_turns = 2025;
    let last_turns = (gears[0] * first_turns) / gears[gears.len()-1];
    return last_turns;
}

pub fn part2(input:String) -> i64 {
    let gears = parse_input(input);
    const EXPECTED:i64 = 10_000_000_000_000;
    let first_turns = ((gears[gears.len()-1] as i64) * EXPECTED - 1) / (gears[0] as i64) + 1;
    return first_turns;
}

fn parse_connected_gears(input:&str) -> (i32, i32) {
    let re = Regex::new(r"(\d+)\|(\d+)").unwrap();
    let captures = re.captures(&input).unwrap();
    return (
        captures[1].parse::<i32>().unwrap(), 
        captures[2].parse::<i32>().unwrap(),
    )
}

pub fn part3(input:String) -> i64 {
    let lines = input.lines().collect::<Vec<_>>();
    let first_gear = lines[0].parse::<i64>().unwrap();
    let last_gear = lines[lines.len()-1].parse::<i64>().unwrap();
    let mut multiplier:i64 = 1;
    for i in 1..lines.len()-1 {
        let (a, b)= parse_connected_gears(lines[i]);
        multiplier *= (b / a) as i64;
    }
    return first_gear * 100 * multiplier / last_gear;
}

mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(String::from("5\n6")), 1687);
        assert_eq!(part1(String::from("128\n64\n32\n16\n8")), 32400);
        assert_eq!(part1(String::from("102\n75\n50\n35\n13")), 15888);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(String::from("128\n64\n32\n16\n8")), 625000000000);
        assert_eq!(part2(String::from("102\n75\n50\n35\n13")), 1274509803922);
    }

    #[test]
    fn test_parse_connected_gears() {
        assert_eq!(parse_connected_gears("7|21"), (7, 21));
    }

    #[test]
    fn test_part3() {
        assert_eq!(part3(String::from("5\n5|10\n10|20\n5")), 400);
        assert_eq!(part3(String::from("5\n7|21\n18|36\n27|27\n10|50\n10|50\n11")), 6818);
    }
}