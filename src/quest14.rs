use array2d::Array2D;

fn parse_input(input: String) -> Array2D<char> {
    let mut data = Vec::new();
    for line in input.lines() {
        data.push(line.chars().collect::<Vec<_>>());
    }
    return Array2D::from_rows(&data).unwrap();
}

fn count_active(data: Array2D<char>) -> i32 {
    let mut count = 0;
    for row_iter in data.rows_iter() {
        for element in row_iter {
            if *element == '#' {
                count += 1;
            }
        }
    }
    return count;
}

fn is_active(data: &Array2D<char>, (i, j): (i32, i32)) -> bool {
    let neighbors = [(-1, -1), (1, -1), (-1, 1), (1, 1)];
    let mut count = 0;
    if data[(i as usize, j as usize)] == '#' {
        count += 1;
    }
    for neighbor in neighbors {
        if 0 > i + neighbor.0 || i + neighbor.0 >= data.num_rows() as i32 ||
            0 > j + neighbor.1 || j + neighbor.1 >= data.num_columns() as i32 {
            continue;
        }
        if data[((i + neighbor.0) as usize, (j + neighbor.1) as usize)] == '#' {
            count += 1;
        }
    }
    return count % 2 == 0;
}

pub fn part1(input: String) -> i32 {
    let mut data = parse_input(input);
    let mut new_data = Array2D::filled_with('.', data.num_rows(), data.num_columns());
    let mut ans = 0;
    for round in 0..10 {
        for i in 0..data.num_rows() {
            for j in 0..data.num_columns() {
                if is_active(&data, (i as i32, j as i32)) {
                    new_data[(i, j)] = '#';
                    ans += 1;
                } else {
                    new_data[(i, j)] = '.';
                }
            }
        }
        data = new_data.clone();
    }
    return ans;
}

pub fn part2(input: String) -> i32 {
    let mut data = parse_input(input);
    let mut new_data = Array2D::filled_with('.', data.num_rows(), data.num_columns());
    let mut ans = 0;
    for round in 0..2025 {
        for i in 0..data.num_rows() {
            for j in 0..data.num_columns() {
                if is_active(&data, (i as i32, j as i32)) {
                    new_data[(i, j)] = '#';
                    ans += 1;
                } else {
                    new_data[(i, j)] = '.';
                }
            }
        }
        data = new_data.clone();
    }
    return ans;
}

pub fn part3(input: String) -> i32 {
    let mut data = parse_input(input);
    let full_data = Array2D::filled_with('.', 34, 34);
    let rounds = 1_000_000_000;
    return 0;
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