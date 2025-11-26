use std::cmp::min;
use std::cmp::max;
use std::cmp::Reverse;
use std::collections::HashMap;
use priority_queue::PriorityQueue;

fn parse_input(input: String) -> Vec<(i32, i32)> {
    let mut coords_x = 0;
    let mut coords_y = 0;
    let mut corners = vec![(0, 0)];
    let dir_vec = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];
    let mut cur_dir: i32 = -1;
    for instruction in input.split(',') {
        let dir = instruction.chars().nth(0).unwrap();
        let (_, num) = instruction.split_at(1);
        let amount: i32 = num.parse().unwrap();
        if dir == 'R' {
            cur_dir = (cur_dir + 1) % dir_vec.len() as i32;
        } else {
            cur_dir = (cur_dir - 1 + dir_vec.len() as i32) % dir_vec.len() as i32;
        }
        coords_x += dir_vec[cur_dir as usize].0 * amount;
        coords_y += dir_vec[cur_dir as usize].1 * amount;
        corners.push((coords_x, coords_y));
    }
    return corners;
}

fn distance((x1, y1): (i32, i32), (x2, y2): (i32, i32)) -> i32 {
    return (x1 - x2).abs() + (y1 - y2).abs();
}

fn cross(x1: (i32, i32), x2: (i32, i32)) -> i64 {
    return x1.0 as i64 * x2.1 as i64 - x1.1 as i64 * x2.0 as i64;
}

fn minus(x1: (i32, i32), x2: (i32, i32)) -> (i32, i32) {
    return (x2.0 - x1.0, x2.1 - x1.1);
}

fn sign(a: i64) -> i64 {
    if a < 0 {
        return -1;
    } else if a > 0 {
        return 1;
    } else {
        return 0;
    }
}

fn intersect(p1: (i32, i32), p2:(i32, i32), p3:(i32, i32), p4:(i32, i32)) -> bool {
    return sign(cross(minus(p3, p1), minus(p2, p1))) * sign(cross(minus(p4, p1), minus(p2, p1))) <= 0 
        && sign(cross(minus(p1, p3), minus(p4, p3))) * sign(cross(minus(p2, p3), minus(p4, p3))) <= 0 
        && max(min(p1.0, p2.0), min(p3.0, p4.0)) <= min(max(p1.0, p2.0), max(p3.0, p4.0))
        && max(min(p1.1, p2.1), min(p3.1, p4.1)) <= min(max(p1.1, p2.1), max(p3.1, p4.1));
}

fn hits_wall((x1, y1): (i32, i32), (x2, y2): (i32, i32), corners: &Vec<(i32, i32)>) -> bool {
    for i in 1..corners.len() {
        if intersect((x1, y1), (x2, y2), corners[i-1], corners[i]) {
            return true;
        }
    }
    return false;
}

pub fn part1(input: String) -> i32 {
    let corners = parse_input(input);
    let mut distances = HashMap::new();
    let mut queue =  PriorityQueue::new();
    let end_shifts = [(-1, 0), (1, 0), (0, 1), (0, -1)];
    queue.push((0, -1), Reverse(1));
    queue.push((0, 1), Reverse(1));
    while !queue.is_empty() {
        let ((x, y), dist) = queue.pop().unwrap();
        if distances.contains_key(&(x, y)) {
            continue;
        }
        distances.insert((x, y), dist.0);
        for i in 0..corners.len()-1 {
            let corner = corners[i];
            let shifts = [(-1, -1), (-1, 1), (1, -1), (1, 1)];
            for shift in shifts {
                let (new_x, new_y) = (corner.0 + shift.0, corner.1 + shift.1);
                if !distances.contains_key(&(new_x, new_y)) && !hits_wall((x, y), (new_x, new_y), &corners) {
                    let new_dist = dist.0 + distance((x, y), (new_x, new_y));
                    if queue.contains(&(new_x, new_y)) {
                        let old_dist = queue.get_priority(&(new_x, new_y)).unwrap().0;
                        queue.change_priority(&(new_x, new_y), Reverse(min(old_dist, new_dist)));
                    } else {
                        queue.push((new_x, new_y), Reverse(new_dist));
                    }
                }
            }
        }
        // Exit point separately
        let corner = corners[corners.len() - 1];
        for shift in end_shifts {
            let (new_x, new_y) = (corner.0 + shift.0, corner.1 + shift.1);
            if !distances.contains_key(&(new_x, new_y)) && !hits_wall((x, y), (new_x, new_y), &corners) {
                let new_dist = dist.0 + distance((x, y), (new_x, new_y));
                if queue.contains(&(new_x, new_y)) {
                    let old_dist = queue.get_priority(&(new_x, new_y)).unwrap().0;
                    queue.change_priority(&(new_x, new_y), Reverse(min(old_dist, new_dist)));
                } else {
                    queue.push((new_x, new_y), Reverse(new_dist));
                }
            }
        }
    }
    let mut ans = 1_000_000_000;
    for shift in end_shifts {
        let (new_x, new_y) = (corners[corners.len()-1].0 + shift.0, corners[corners.len()-1].1 + shift.1);
        let possible_ans = *distances.entry((new_x, new_y)).or_insert(1_000_000_000);
        //println!("{} {} {}", new_x, new_y, possible_ans);
        ans = min(ans, possible_ans + 1);
    }
    return ans;
}

mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(String::from("R3,R4,L3,L4,R3,R6,R9")), 6);
        assert_eq!(part1(String::from("L6,L3,L6,R3,L6,L3,L3,R6,L6,R6,L6,L6,R3,L3,L3,R3,R3,L6,L6,L3")), 16);
    }
}