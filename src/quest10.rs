use std::collections::{HashMap, VecDeque};

fn find_dragon(data: &Vec<Vec<char>>) -> (i32, i32) {
    for i in 0..data.len() {
        for j in 0..data[0].len() {
            if data[i][j] == 'D' {
                return (i as i32, j as i32);
            }
        }
    }
    return (-1, -1);
}

fn eat_sheep(data: &mut Vec<Vec<char>>, moves: i32) -> i32 {
    let (dr_x, dr_y) = find_dragon(data);
    let mut queue =  VecDeque::from([(dr_x, dr_y, 0)]);
    let mut eaten = 0;
    while !queue.is_empty() {
        let (i, j, depth) = queue.pop_front().unwrap();
        for dx in -2..=2i32 {
            for dy in -2..=2i32 {
                if dx.abs() + dy.abs() != 3 {
                    continue;
                }
                if 0 > i + dx || i + dx > data.len() as i32 ||
                   0 > j + dy || j + dy > data[0].len() as i32{
                    continue;
                }
                if depth + 1 > moves {
                    continue;
                }
                match data[(i+dx) as usize][(j+dy) as usize] {
                    'S' => {
                        eaten += 1;
                        data[(i+dx) as usize][(j+dy) as usize] = 'X';
                    },
                    'X' => continue,
                    _ => data[(i+dx) as usize][(j+dy) as usize] = 'X',
                }
                queue.push_back((i+dx, j+dy, depth+1));
            }
        }
    }
    return eaten;
}

fn split_hides(data: &mut Vec<Vec<char>>) -> Vec<Vec<bool>> {
    let mut hides = Vec::new();
    for i in 0..data.len() {
        let mut hides_i = Vec::new();
        for j in 0..data[0].len() {
            hides_i.push(data[i][j] == '#');
            if data[i][j] == '#' {
                data[i][j] = '.';
            }
        }
        hides.push(hides_i);
    }
    return hides;
}

fn eat_sheep_move(data: &mut Vec<Vec<char>>, turns: i32) -> i32 {
    let (dr_x, dr_y) = find_dragon(data);
    let mut queue: VecDeque<(i32, i32)> =  VecDeque::from([(dr_x, dr_y)]);
    let mut eaten = 0;
    let hides = split_hides(data);
    for turn in 0..=turns { 
        let mut next_queue: VecDeque<(i32, i32)> =  VecDeque::new();
        while !queue.is_empty() {
            let (i, j) = queue.pop_front().unwrap();
            if 0 <= i - turn && i - turn < data.len() as i32 {
                if data[(i-turn) as usize][(j) as usize] == 'S' &&  
                    !hides[(i) as usize][(j) as usize]
                {
                    eaten += 1;
                    data[(i-turn) as usize][(j) as usize] = '.';
                }
            }
            if turn == turns {
                continue;
            }
            for dx in -2..=2i32 {
                for dy in -2..=2i32 {
                    if dx.abs() + dy.abs() != 3 {
                        continue;
                    }
                    if 0 > i + dx || i + dx >= data.len() as i32 ||
                    0 > j + dy || j + dy >= data[0].len() as i32{
                        continue;
                    }
                    if 0 <= i - turn + dx && i - turn + dx < data.len() as i32 {
                        if data[(i-turn+dx) as usize][(j+dy) as usize] == 'S' &&  
                            !hides[(i+dx) as usize][(j+dy) as usize]
                        {
                            eaten += 1;
                            data[(i-turn+dx) as usize][(j+dy) as usize] = '.';
                        }
                    }
                    if next_queue.contains(&(i+dx, j+dy)) {
                        continue;
                    }
                    next_queue.push_back((i+dx, j+dy));
                }
            }
        }
        queue = next_queue;
    }
    return eaten;
}

fn parse_input(input: String) -> Vec<Vec<char>>{
    let mut data = Vec::new();
    for line in input.lines() {
        data.push(line.chars().collect::<Vec<_>>());
    }
    return data;
}

pub fn part1(input: String) -> i32 {
    let mut data = parse_input(input);
    return eat_sheep(&mut data, 4);
}

pub fn part2(input: String) -> i32 {
    let mut data = parse_input(input);
    return eat_sheep_move(&mut data, 20);
}

fn find_sheep(data: &Vec<Vec<char>>) -> Vec<(usize, usize)>{
    let mut sheep: Vec<(usize, usize)> = Vec::new();
    'j: for j in 0..data[0].len() {
        for i in 0..data.len() {
            if data[i][j] == 'S' {
                sheep.push((i, j));
                continue 'j;
            }
        }
    }
    return sheep;
}

fn calc_win(board_x: usize, board_y: usize, 
        dr_x: usize, dr_y: usize, 
        sheep: &mut Vec<(usize, usize)>, 
        calculated: &mut HashMap<(usize, usize, Vec<(usize, usize)>), i64>, 
        hides: &Vec<Vec<bool>>) -> i64 {
    /*println!("fn {} {}", dr_x, dr_y);
    for i in 0..sheep.len() {
        println!("S {} {} ", sheep[i].0, sheep[i].1);
    }*/
    if sheep.is_empty() {
        return 1;
    }
    if calculated.contains_key(&(dr_x, dr_y, sheep.to_vec())) {
        return *calculated.get(&(dr_x, dr_y, sheep.to_vec())).unwrap();
    }
    let mut ans: i64 = 0;
    let mut had_turn = false;
    for i in 0..sheep.len() {
        if sheep[i].0 + 1 >= board_x {
            had_turn = true;
            continue;
        }
        if sheep[i].0 + 1 == dr_x 
            && sheep[i].1 == dr_y 
            && !hides[dr_x][dr_y] {
            continue;
        }
        sheep[i].0 += 1;
        had_turn = true;
        //println!("Sheep: {} {} {}", i, sheep[i].0, sheep[i].1);
        // Dragon's turn
        for dx in -2..=2i32 {
            for dy in -2..=2i32 {
                if dx.abs() + dy.abs() != 3 {
                    continue;
                }
                if 0 > dr_x as i32 + dx || dr_x as i32 + dx >= board_x as i32 ||
                0 > dr_y as i32 + dy || dr_y as i32 + dy >= board_y as i32 {
                    continue;
                }
                // Valid dragon move
                //println!("Dragon: {} {}", dr_x as i32 + dx, dr_y as i32 + dy);
                let mut alive_sheep = Vec::new();
                for j in 0..sheep.len() {
                    if dr_x as i32 + dx == sheep[j].0 as i32 
                        && dr_y as i32 + dy == sheep[j].1 as i32 
                        && !hides[(dr_x as i32 + dx) as usize][(dr_y as i32 + dy) as usize]{
                       continue;
                    }
                    alive_sheep.push(sheep[j]);
                }
                ans += calc_win(board_x, board_y, 
                    (dr_x as i32 + dx) as usize, 
                    (dr_y as i32 + dy) as usize,
                    &mut alive_sheep, calculated, hides);
                //println!("add {} {} {} {}", dr_x, dr_y, sheep.len(), ans);
            }
        }
        sheep[i].0 -= 1;
    }
    if !had_turn {
        //println!("{}: {} {}", sheep.len(), sheep[0].0, sheep[0].1);
        // Dragon's turn
        for dx in -2..=2i32 {
            for dy in -2..=2i32 {
                if dx.abs() + dy.abs() != 3 {
                    continue;
                }
                if 0 > dr_x as i32 + dx || dr_x as i32 + dx >= board_x as i32 ||
                0 > dr_y as i32 + dy || dr_y as i32 + dy >= board_y as i32 {
                    continue;
                }
                // Valid dragon move
                //println!("Dragon: {} {}", dr_x as i32 + dx, dr_y as i32 + dy);
                let mut alive_sheep = Vec::new();
                for j in 0..sheep.len() {
                    if dr_x as i32 + dx == sheep[j].0 as i32 
                        && dr_y as i32 + dy == sheep[j].1 as i32 
                        && !hides[(dr_x as i32 + dx) as usize][(dr_y as i32 + dy) as usize]{
                       continue;
                    }
                    alive_sheep.push(sheep[j]);
                }
                //println!("{}", alive_sheep.len());
                ans += calc_win(board_x, board_y, 
                    (dr_x as i32 + dx) as usize, 
                    (dr_y as i32 + dy) as usize,
                    &mut alive_sheep, calculated, hides);
                //println!("add {} {} {} {}", dr_x, dr_y, sheep.len(), ans);
            }
        }
    }
    //println!("{} {} {} {}", dr_x, dr_y, sheep.len(), ans);
    calculated.insert((dr_x, dr_y, sheep.clone()), ans);
    return ans;
}

pub fn part3(input: String) -> i64 {
    let mut data = parse_input(input);
    let (dr_x, dr_y) = find_dragon(&data);
    let hides = split_hides(&mut data);
    let mut sheep = find_sheep(&data);
    let mut calculated = HashMap::new();
    return calc_win(data.len(), data[0].len(),
        dr_x as usize, dr_y as usize,
        &mut sheep,
        &mut calculated, 
        &hides);
}

mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(eat_sheep(&mut parse_input(String::from("...SSS.......
.S......S.SS.
..S....S...S.
..........SS.
..SSSS...S...
.....SS..S..S
SS....D.S....
S.S..S..S....
....S.......S
.SSS..SS.....
.........S...
.......S....S
SS.....S..S..")), 3), 27);
    }

    #[test]
    fn test_part2() {
        assert_eq!(eat_sheep_move(&mut parse_input(String::from("...SSS##.....
.S#.##..S#SS.
..S.##.S#..S.
.#..#S##..SS.
..SSSS.#.S.#.
.##..SS.#S.#S
SS##.#D.S.#..
S.S..S..S###.
.##.S#.#....S
.SSS.#SS..##.
..#.##...S##.
.#...#.S#...S
SS...#.S.#S..")), 3), 27);
    }

    #[test]
    fn test_part3() {
        assert_eq!(part3(String::from("SSS
..#
#.#
#D.")), 15);
        assert_eq!(part3(String::from("SSS
..#
..#
.##
.D#")), 8);
        assert_eq!(part3(String::from("..S..
.....
..#..
.....
..D..")), 44);
        assert_eq!(part3(String::from(".SS.S
#...#
...#.
##..#
.####
##D.#")), 4406);
        assert_eq!(part3(String::from("SSS.S
.....
#.#.#
.#.#.
#.D.#")), 13_033_988_838);
    }
}