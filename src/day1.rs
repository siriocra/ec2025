use std::cmp::max;
use std::cmp::min;

pub fn part1(input:String) {
    let split = input.split("\n").collect::<Vec<_>>();
    let names = split[0].split(",").collect::<Vec<_>>();
    let movements = split[2].split(",");
    let max_ind = names.len() as i32 - 1;

    let mut index = 0;
    for movement in movements {
        let movement_dir = movement.chars().next().unwrap();
        let index_diff_chr = &movement[1..movement.len()];
        let index_diff = index_diff_chr.parse::<i32>().unwrap();

        let mut dir = 1;
        if movement_dir == 'L' {
            dir = -1;
        }
        index += dir * index_diff;
        index = max(index, 0);
        index = min(index, max_ind);
    }
    let uindex:usize = index.try_into().unwrap();
    println!("Day 1 Part 1: {}", names[uindex]);
}

pub fn part2(input:String) {
    let split = input.split("\n").collect::<Vec<_>>();
    let names = split[0].split(",").collect::<Vec<_>>();
    let movements = split[2].split(",");
    let max_ind = names.len() as i32 - 1;

    let mut index = 0;
    for movement in movements {
        let movement_dir = movement.chars().next().unwrap();
        let index_diff_chr = &movement[1..movement.len()];
        let index_diff = index_diff_chr.parse::<i32>().unwrap();

        let mut dir = 1;
        if movement_dir == 'L' {
            dir = -1;
        }
        index += dir * index_diff;
        index = (index + max_ind + 1) % (max_ind + 1);
    }
    let uindex:usize = index.try_into().unwrap();
    println!("Day 1 Part 2: {}", names[uindex]);
}

pub fn part3(input:String) {
    let split = input.split("\n").collect::<Vec<_>>();
    let mut names = split[0].split(",").collect::<Vec<_>>();
    let movements = split[2].split(",");
    let max_ind = names.len() as i32 - 1;

    for movement in movements {
        let movement_dir = movement.chars().next().unwrap();
        let index_diff_chr = &movement[1..movement.len()];
        let index_diff = index_diff_chr.parse::<i32>().unwrap();

        let mut dir = 1;
        if movement_dir == 'L' {
            dir = -1;
        }
        let mut index = (dir * index_diff) % (max_ind + 1);
        index = (index + max_ind + 1) % (max_ind + 1);
        names.swap(0, index as usize);
    }
    println!("Day 1 Part 3: {}", names[0]);
}