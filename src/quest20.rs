use std::{cmp::{Reverse, min}, collections::VecDeque};

use array2d::Array2D;
use priority_queue::PriorityQueue;

fn parse_input(input: String) -> (Array2D<char>, (usize, usize), (usize, usize)) {
    let mut data = Vec::new();
    let mut start = (0, 0);
    let mut end = (0, 0);
    for line in input.lines() {
        if line.contains('S') {
            let y = line.find('S').unwrap();
            start = (data.len(), y);
        }
        if line.contains('E') {
            let y = line.find('E').unwrap();
            end = (data.len(), y);
        }
        data.push(line.replace('S', "T").replace('E', "T").chars().collect::<Vec<_>>());
    }
    return (Array2D::from_rows(&data).unwrap(), start, end);
}

fn calc_pairs(data: &Array2D<char>) -> i32 {
    let mut calc_neighbors = 0;
    let w = data.row_len();
    for i in 0..w {
        for j in i..w - i {
            if data[(i, j)] != 'T' {
                continue;
            }
            let mut neighbors: Vec<(i32, i32)> = vec![];
            if (i + j) % 2 == 0 {
                // Upside down
                neighbors = vec![(0, -1), (0, 1), (-1, 0)];
            } else {
                neighbors = vec![(0, -1), (0, 1), (1, 0)];
            }
            for (dx, dy) in neighbors {
                if 0 > i as i32 + dx || i as i32 + dx >= w as i32
                    || i as i32 > j as i32 + dy || j as i32 + dy >= (w - i) as i32 {
                    continue;
                }
                let (new_x, new_y) = ((i as i32 + dx) as usize, (j as i32 + dy) as usize);
                if data[(new_x, new_y)] == 'T' {
                    calc_neighbors += 1;
                }
            }
        }
    }
    return calc_neighbors / 2;
}

fn bfs(start: (usize, usize), end: (usize, usize), data: Vec<&Array2D<char>>) -> i32 {
    let mut queue: PriorityQueue<(usize, (usize, usize)), Reverse<i32>> = PriorityQueue::new();
    let w = data[0].row_len();
    let h = data[0].num_rows();
    queue.push((0, start), Reverse(0));
    let mut distance: [Array2D<i32>; 3] = [
        Array2D::filled_with(-1, h, w),
        Array2D::filled_with(-1, h, w),
        Array2D::filled_with(-1, h, w),
    ];

    let mut visited = [
        Array2D::filled_with(false, h, w),
        Array2D::filled_with(false, h, w),
        Array2D::filled_with(false, h, w),
    ];
    distance[0][start] = 0;
    while !queue.is_empty() {
        let ((plane, v), dist) = queue.pop().unwrap();
        let next_p = (plane + 1) % 3;
        visited[plane][v] = true;
        let mut neighbors: Vec<(i32, i32)> = vec![];
        if (v.0 + v.1) % 2 == 0 {
            // Upside down
            neighbors = vec![(0, 0), (0, -1), (0, 1), (-1, 0)];
        } else {
            neighbors = vec![(0, 0), (0, -1), (0, 1), (1, 0)];
        }
        for (dx, dy) in neighbors {
            if 0 > v.0 as i32 + dx || v.0 as i32 + dx >= w as i32
                || v.0 as i32 > v.1 as i32 + dy || v.1 as i32 + dy >= (w - v.0) as i32 {
                continue;
            }
            let new_v = ((v.0 as i32 + dx) as usize, (v.1 as i32 + dy) as usize);
            if data[next_p][new_v] == 'T' && !visited[next_p][new_v] {
                if queue.contains(&(next_p, new_v)) {
                    distance[next_p][new_v] = min(distance[next_p][new_v], distance[plane][v] + 1);
                    queue.change_priority(&(next_p, new_v), Reverse(distance[next_p][new_v]));
                } else {
                    distance[next_p][new_v] = distance[plane][v] + 1;
                    queue.push((next_p, new_v), Reverse(distance[next_p][new_v]));
                }
            }
        }
    }
    let mut ans: i32 = 1000000;
    let mut new_end = end;
    for d in distance {
        if d[new_end] != -1 {
            println!("{} {} {}", new_end.0, new_end.1, d[new_end]);
            ans = min(ans, d[new_end]);
        }
        new_end = rotate_end(new_end, w);
    }
    return ans;
}

fn rotate_end((i, j): (usize, usize), w: usize) -> (usize, usize) {
    return ((j - i) / 2, w - (j - i + 1) / 2 - 2 * i - 1);
}

fn rotate(data: &Array2D<char>) -> Array2D<char> {
    let w = data.row_len();
    let h = data.num_rows();
    let mut new_data = Array2D::filled_with('.', h, w);
    for i in 0..h {
        for j in i..w - i {
            new_data[((j - i) / 2, w - (j - i + 1) / 2 - 2 * i - 1)] = data[(i, j)];
        }
    }
    return new_data;
}

pub fn part1(input: String) -> i32 {
    let (data, _, _) = parse_input(input);
    return calc_pairs(&data);
}

pub fn part2(input: String) -> i32 {
    let (data, start, end) = parse_input(input);
    return bfs(start, end, vec![&data, &data, &data]);
}

pub fn part3(input: String) -> i32 {
    let (data, start, end) = parse_input(input);
    let data_2 = rotate(&data);
    let data_3 = rotate(&data_2);
    return bfs(start, end, vec![&data, &data_2, &data_3]);
}

mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(String::from("T#TTT###T##
.##TT#TT##.
..T###T#T..
...##TT#...
....T##....
.....#.....")), 7);
        assert_eq!(part1(String::from("T#T#T#T#T#T
.T#T#T#T#T.
..T#T#T#T..
...T#T#T...
....T#T....
.....T.....")), 0);
        assert_eq!(part1(String::from("T#T#T#T#T#T
.#T#T#T#T#.
..#T###T#..
...##T##...
....#T#....
.....#.....")), 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(String::from("TTTTTTTTTTTTTTTTT
.TTTT#T#T#TTTTTT.
..TT#TTTETT#TTT..
...TT#T#TTT#TT...
....TTT#T#TTT....
.....TTTTTT#.....
......TT#TT......
.......#TT.......
........S........")), 32);
    }

    #[test]
    fn test_part3() {
        assert_eq!(part3(String::from("T####T#TTT##T##T#T#
.T#####TTTT##TTT##.
..TTTT#T###TTTT#T..
...T#TTT#ETTTT##...
....#TT##T#T##T....
.....#TT####T#.....
......T#TT#T#......
.......T#TTT.......
........TT#........
.........S.........")), 23);
    }
}