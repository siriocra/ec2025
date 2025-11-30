use std::cmp::min;
use std::cmp::max;
use std::collections::HashMap;

fn parse_input(input: String) -> Vec<Vec<i32>> {
    let mut openings = Vec::new();
    for line in input.lines() {
        openings.push(line.split(',').map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>());
    }
    return openings;
}

fn calc_flaps((x1, y1): (i32, i32), (x2, y2): (i32, i32)) -> i64 {
    let change_x = x2 - x1;
    let change_y = y2 - y1;
    let mut flaps = 0;
    if change_x > change_y.abs() {
        flaps += (change_x - change_y.abs() + 1) / 2;
    }
    if change_y > 0 {
        flaps += change_y;
    }
    return flaps as i64;
}

fn can_fly((x1, y1): (i32, i32), (x2, y2): (i32, i32)) -> bool {
    return (y2 - y1).abs() <= x2 - x1;
}

pub fn part1(input: String) -> i64 {
    let openings = parse_input(input);
    // Already sorted by x
    let mut cur_x = 0;
    let mut cur_min_y = 0;
    let mut cur_max_y = 0;
    let mut cache: HashMap<(i32, i32), i64> = HashMap::new();
    let mut ans_flaps = 0;
    cache.insert((0, 0), 0);
    for opening in openings {
        let (new_x, y, height) = (opening[0], opening[1], opening[2]);
        println!("{} {} {}", new_x, y, height);
        // Possible range
        let mut new_min_y = max(cur_min_y - (new_x - cur_x), y);
        if new_min_y % 2 != new_x % 2 {
            new_min_y += 1;
        }
        let mut new_max_y = min(cur_max_y + (new_x - cur_x), y + height);
        if new_max_y % 2 != new_x % 2 {
            new_max_y -= 1;
        }

        let mut min_flaps = 1_000_000_000;
        for new_y in (new_min_y..=new_max_y).step_by(2) {
            for cur_y in (cur_min_y..=cur_max_y).step_by(2) {
                if can_fly((cur_x, cur_y), (new_x, new_y)) {
                    let old_flaps = *cache.get(&(cur_x, cur_y)).unwrap();
                    let flaps = calc_flaps((cur_x, cur_y), (new_x, new_y));
                    cache.entry((new_x, new_y)).and_modify(|x| *x = min(*x, old_flaps + flaps)).or_insert(old_flaps + flaps);
                    min_flaps = min(min_flaps, old_flaps + flaps);
                }
            }
        }
        cur_min_y = new_min_y;
        cur_max_y = new_max_y;
        
        cur_x = new_x;
        ans_flaps = min_flaps;
    }
    return ans_flaps;
}

pub fn part2(input: String) -> i64 {
    // Part 3 same as Part 2
    let openings = parse_input(input);
    // Already sorted by x
    let mut cur_x = 0;
    let mut cur_ranges = vec![(0, 0)];
    let mut next_ranges = Vec::new();
    let mut cache: HashMap<(i32, i32), i64> = HashMap::new();
    let mut ans_flaps = 1_000_000_000;
    cache.insert((0, 0), 0);
    for i in 0..openings.len() {
        let opening = &openings[i];
        let (new_x, y, height) = (opening[0], opening[1], opening[2]);
        
        let mut min_flaps = 1_000_000_000;
        for (cur_min_y, cur_max_y) in &cur_ranges {
            // Possible range
            let mut new_min_y = max(cur_min_y - (new_x - cur_x), y);
            if new_min_y % 2 != new_x % 2 {
                new_min_y += 1;
            }
            let mut new_max_y = min(cur_max_y + (new_x - cur_x), y + height);
            if new_max_y % 2 != new_x % 2 {
                new_max_y -= 1;
            }

            for new_y in (new_min_y..=new_max_y).step_by(2) {
                for cur_y in (*cur_min_y..=*cur_max_y).step_by(2) {
                    if can_fly((cur_x, cur_y), (new_x, new_y)) {
                        let old_flaps = *cache.get(&(cur_x, cur_y)).unwrap();
                        let flaps = calc_flaps((cur_x, cur_y), (new_x, new_y));
                        cache.entry((new_x, new_y)).and_modify(|x| *x = min(*x, old_flaps + flaps)).or_insert(old_flaps + flaps);
                        min_flaps = min(min_flaps, old_flaps + flaps);
                    }
                }
            }
            if !next_ranges.contains(&(new_min_y, new_max_y)) && min_flaps != 1_000_000_000 {
                next_ranges.push((new_min_y, new_max_y));
            }
        }
        if i + 1 < openings.len() && openings[i + 1][0] == openings[i][0] {
            // Same coordinate
            if new_x == openings[openings.len()-1][0] {
                ans_flaps = min(ans_flaps, min_flaps);
            }
        } else {
            cur_ranges = next_ranges.clone();
            next_ranges = Vec::new();
            cur_x = new_x;
            if new_x == openings[openings.len()-1][0] {
                ans_flaps = min(ans_flaps, min_flaps);
            }
        }
        
    }
    return ans_flaps;
}

mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(String::from("7,7,2
12,0,4
15,5,3
24,1,6
28,5,5
40,8,2")), 24);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(String::from("7,7,2
7,1,3
12,0,4
15,5,3
24,1,6
28,5,5
40,3,3
40,8,2")), 22);
    }

    #[test]
    fn test_flaps() {
        assert_eq!(calc_flaps((7, 7), (12, 6)), 2);
        assert_eq!(calc_flaps((7, 7), (11, 7)), 2);
        assert_eq!(calc_flaps((7, 7), (11, 9)), 3);
        assert_eq!(calc_flaps((7, 7), (10, 10)), 3);
        assert_eq!(calc_flaps((7, 7), (11, 11)), 4);
        assert_eq!(calc_flaps((7, 7), (9, 5)), 0);
        assert_eq!(calc_flaps((7, 7), (10, 6)), 1);
    }
}