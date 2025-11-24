use std::collections::VecDeque;
use array2d::Array2D;

fn parse_input(input: String) -> Vec<Vec<u32>>{
    let mut data: Vec<Vec<u32>> = Vec::new();
    for line in input.lines() {
        data.push(line.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<_>>());
    }
    return data;
}

fn bfs(visited: &mut Array2D<i32>, data: &Vec<Vec<u32>>, start: Vec<(usize, usize)>, component: i32) {
    let neighbors: Vec<(i32, i32)> = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut queue: VecDeque<(usize, usize)> =  VecDeque::from(start);
    while !queue.is_empty() {
        let (x, y) = queue.pop_front().unwrap();
        visited[(x, y)] = component;
        for (dx, dy) in &neighbors {
            let (new_x, new_y) = (x as i32 + dx, y as i32 + dy);
            if 0 > new_x || new_x >= data.len() as i32 ||
                0 > new_y || new_y >= data[0].len() as i32{
                continue;
            }
            if visited[(new_x as usize, new_y as usize)] == 0 
                && data[x as usize][y as usize] >= data[new_x as usize][new_y as usize] {
                queue.push_back((new_x as usize, new_y as usize));
                visited[(new_x as usize, new_y as usize)] = component;
            }
        }
    }
}

fn calc_destroyed(visited: &Array2D<i32>, component: i32) -> i32 {
    let mut ans = 0;
    for i in 0..visited.column_len() {
        for j in 0..visited.row_len() {
            if visited[(i, j)] == component {
                ans += 1;
            }
        }
    }
    return ans;
}

pub fn part1(input: String) -> i32 {
    let data = parse_input(input);
    let mut visited = Array2D::filled_with(0, data.len(), data[0].len());
    bfs(&mut visited, &data, vec![(0 as usize, 0 as usize)], 1);
    return calc_destroyed(&visited, 1);
}

pub fn part2(input: String) -> i32 {
    let data = parse_input(input);
    let mut visited = Array2D::filled_with(0, data.len(), data[0].len());
    bfs(&mut visited, &data, vec![
        (0 as usize, 0 as usize),
        (data.len()-1, data[0].len()-1),
    ], 1);
    return calc_destroyed(&visited, 1);
}

pub fn part3(input: String) -> i32 {
    let data = parse_input(input);
    let mut ans = 0;
    let mut max_destroyed = 0;
    let mut first_best = (0, 0);
    let mut component = 1;
    for i in 0..data.len() {
        for j in 0..data[0].len() {
            let mut visited = Array2D::filled_with(0, data.len(), data[0].len());
            bfs(&mut visited, &data, vec![(i, j)], component);
            let destroyed = calc_destroyed(&visited, component);
            if destroyed > max_destroyed {
                max_destroyed = destroyed;
                first_best = (i, j);
            }
        }
    }
    println!("{}", max_destroyed);

    let mut visited_first = Array2D::filled_with(0, data.len(), data[0].len());
    bfs(&mut visited_first, &data, vec![first_best], component);
    component += 1;
    ans += max_destroyed;

    max_destroyed = 0;
    let mut second_best = (0, 0);
    // Second run
    for i in 0..data.len() {
        for j in 0..data[0].len() {
            if visited_first[(i, j)] != 0 {
                continue;
            }
            let mut visited = visited_first.clone();
            bfs(&mut visited, &data, vec![(i, j)], component);
            let destroyed = calc_destroyed(&visited, component);
            if destroyed > max_destroyed {
                max_destroyed = destroyed;
                second_best = (i, j);
            }
        }
    }
    println!("{}", max_destroyed);

    let mut visited_second = visited_first.clone();
    bfs(&mut visited_second, &data, vec![second_best], component);
    component += 1;
    ans += max_destroyed;

    max_destroyed = 0;
    let mut third_best = (0, 0);
    // Third run
    for i in 0..data.len() {
        for j in 0..data[0].len() {
            if visited_second[(i, j)] != 0 {
                continue;
            }
            let mut visited = visited_second.clone();
            bfs(&mut visited, &data, vec![(i, j)], component);
            let destroyed = calc_destroyed(&visited, component);
            if destroyed > max_destroyed {
                max_destroyed = destroyed;
                third_best = (i, j);
            }
        }
    }
    println!("{}", max_destroyed);

    println!("1: {} {} 2: {} {} 3: {} {}", first_best.0, first_best.1, second_best.0, second_best.1, third_best.0, third_best.1);
    let mut visited = Array2D::filled_with(0, data.len(), data[0].len());
    bfs(&mut visited, &data, vec![first_best, second_best, third_best], 1);
    return calc_destroyed(&visited, 1);
}

mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(String::from("989601
857782
746543
766789")), 16);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(String::from("9589233445
9679121695
8469121876
8352919876
7342914327
7234193437
6789193538
6781219648
5691219769
5443329859")), 58);
    }

    #[test]
    fn test_part3() {
        assert_eq!(part3(String::from("5411
3362
5235
3112")), 14);
        assert_eq!(part3(String::from("41951111131882511179
32112222211518122215
31223333322115122219
31234444432147511128
91223333322176121892
61112222211166431583
14661111166111111746
11111119142122222177
41222118881233333219
71222127839122222196
56111126279711111517")), 136);
    }
}