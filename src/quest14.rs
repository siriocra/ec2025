use std::collections::HashMap;

use array2d::Array2D;

fn parse_input(input: String) -> Array2D<bool> {
    let mut data = Vec::new();
    for line in input.lines() {
        data.push(line.chars().map(|x| x == '#').collect::<Vec<_>>());
    }
    return Array2D::from_rows(&data).unwrap();
}

fn count_active(data: Array2D<bool>) -> i32 {
    let mut count = 0;
    for row_iter in data.rows_iter() {
        for element in row_iter {
            if *element == true {
                count += 1;
            }
        }
    }
    return count;
}

fn is_active(data: &Array2D<bool>, (i, j): (i32, i32)) -> bool {
    let neighbors = [(-1, -1), (1, -1), (-1, 1), (1, 1)];
    let mut count = 0;
    if data[(i as usize, j as usize)] {
        count += 1;
    }
    for neighbor in neighbors {
        if 0 > i + neighbor.0 || i + neighbor.0 >= data.num_rows() as i32 ||
            0 > j + neighbor.1 || j + neighbor.1 >= data.num_columns() as i32 {
            continue;
        }
        if data[((i + neighbor.0) as usize, (j + neighbor.1) as usize)] {
            count += 1;
        }
    }
    return count % 2 == 0;
}

pub fn part1(input: String) -> i32 {
    let mut data = parse_input(input);
    let mut new_data = Array2D::filled_with(false, data.num_rows(), data.num_columns());
    let mut ans = 0;
    for round in 0..10 {
        for i in 0..data.num_rows() {
            for j in 0..data.num_columns() {
                if is_active(&data, (i as i32, j as i32)) {
                    new_data[(i, j)] = true;
                    ans += 1;
                } else {
                    new_data[(i, j)] = false;
                }
            }
        }
        data = new_data.clone();
    }
    return ans;
}

pub fn part2(input: String) -> i32 {
    let mut data = parse_input(input);
    let mut new_data = Array2D::filled_with(false, data.num_rows(), data.num_columns());
    let mut ans = 0;
    for round in 0..2025 {
        for i in 0..data.num_rows() {
            for j in 0..data.num_columns() {
                if is_active(&data, (i as i32, j as i32)) {
                    new_data[(i, j)] = true;
                    ans += 1;
                } else {
                    new_data[(i, j)] = false;
                }
            }
        }
        data = new_data.clone();
    }
    return ans;
}

fn contains_subdata(data: &Array2D<bool>, full_data: &Array2D<bool>) -> bool {
    let w = full_data.num_rows();
    let h = full_data.num_columns();
    let start_i = (w - data.num_rows())/2;
    let start_j = (h - data.num_columns())/2;
    for i in start_i..start_i + data.num_rows() {
        for j in start_j..start_j + data.num_columns() {
            if full_data[(i, j)] != data[(i-start_i, j-start_j)] {
                return false;
            }
        }
    }
    return true;
}

pub fn part3(input: String) -> i64 {
    let mut data = parse_input(input);
    let mut full_data = Array2D::filled_with(false, 34, 34);
    let mut new_data = Array2D::filled_with(false, 34, 34);
    let mut cache = HashMap::new();
    let mut active = Vec::new();
    let mut calc_rounds = Vec::new();
    let mut rounds: i64 = 1_000_000_000;
    let mut repeats = 0;
    for round in 0..10000 {
        active.push(0);
        for i in 0..full_data.num_rows() {
            for j in 0..full_data.num_columns() {
                if is_active(&full_data, (i as i32, j as i32)) {
                    new_data[(i, j)] = true;
                    active[round] += 1;
                } else {
                    new_data[(i, j)] = false;
                }
            }
        }
        if contains_subdata(&data, &new_data) {
            calc_rounds.push(round);
        }
        if cache.contains_key(&new_data) {
            repeats = round;
            break;
        }
        cache.insert(new_data.clone(), round);
        //println!("Round {} active {}", round, active);
        full_data = new_data.clone();
    }
    let mut ans: i64 = 0;
    for i in 0..calc_rounds.len() {
        ans += active[calc_rounds[i]];
    }
    ans = ans * (rounds / repeats as i64);
    rounds = rounds % repeats as i64;
    for i in 0..calc_rounds.len() {
        if (calc_rounds[i] as i64) < rounds {
            ans += active[calc_rounds[i]];
        }
    }
    return ans;
}

mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(String::from(".#.##.
##..#.
..##.#
.#.##.
.###..
###.##")), 200);
    }

    #[test]
    fn test_part3() {
        assert_eq!(part3(String::from("#......#
..#..#..
.##..##.
...##...
...##...
.##..##.
..#..#..
#......#")), 278388552);
    }

}