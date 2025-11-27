use std::{cmp::Reverse, collections::{HashMap, VecDeque}};

use priority_queue::PriorityQueue;
use regex::Regex;

fn parse_plant(line: &str) -> (i32, i32) {
    let re = Regex::new(r"Plant (\d+) with thickness (-?\d+):").unwrap();
    let captures = re.captures(&line).unwrap();
    return (
        captures[1].parse::<i32>().unwrap(), 
        captures[2].parse::<i32>().unwrap(),
    )
}

fn parse_free_branch(line: &str) -> i32 {
    let re = Regex::new(r"- free branch with thickness (-?\d+)").unwrap();
    let captures = re.captures(&line).unwrap();
    return captures[1].parse::<i32>().unwrap();
}

fn parse_branch(line: &str) -> (i32, i32) {
    let re = Regex::new(r"- branch to Plant (\d+) with thickness (-?\d+)").unwrap();
    let captures = re.captures(&line).unwrap();
    return (
        captures[1].parse::<i32>().unwrap(), 
        captures[2].parse::<i32>().unwrap(),
    )
}
struct Graph {
    vertices: HashMap<i32, i32>, 
    edges: HashMap<i32, Vec<(i32, i32)>>,
    last_plant: i32,
}

fn parse_input(input: String) -> (Graph, Vec<Vec<i32>>) {
    let mut vertices: HashMap<i32, i32> = HashMap::new();
    let mut edges: HashMap<i32, Vec<(i32, i32)>> = HashMap::new();
    let mut cur_plant = 0;
    let mut test_cases = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        if !line.contains("-") && line.contains("Plant") {
            let (new_plant, thickness) = parse_plant(line);
            vertices.insert(new_plant, thickness);
            cur_plant = new_plant;
        } else if line.contains("free") {
            let thickness= parse_free_branch(line);
            edges.entry(0).or_insert(Vec::new()).push((cur_plant, thickness));
        } else if line.contains("branch"){
            let (new_plant, thickness) = parse_branch(line);
            edges.entry(new_plant).or_insert(Vec::new()).push((cur_plant, thickness));
        } else {
            // test case
            test_cases.push(line.split(' ').map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>());
        }
    }
    vertices.insert(0, 1);
    return (Graph{vertices: vertices, edges: edges, last_plant: cur_plant}, test_cases);
}

fn activate(plant: i32, graph: &mut Graph, test: Option<&Vec<i32>>) -> i32 {
    let mut queue = PriorityQueue::new();
    let mut cur_energy = HashMap::new();
    if test.is_none() {
        queue.push(plant, Reverse(1));
        cur_energy.insert(plant, 1);
    } else {
        for (new_v, thickness) in graph.edges.get(&0).unwrap() {
            if test.unwrap()[(*new_v - 1) as usize] == 1 {
                *cur_energy.entry(*new_v).or_insert(0) += *thickness;
                if cur_energy.get(new_v).unwrap() >= graph.vertices.get(new_v).unwrap() {
                    queue.push(*new_v, Reverse(*cur_energy.get(new_v).unwrap()));
                }
            }
        }
    }
    while !queue.is_empty() {
        let (v, _) = queue.pop().unwrap();
        if *cur_energy.get(&v).unwrap() < *graph.vertices.get(&v).unwrap() {
            cur_energy.entry(v).and_modify(|e| { *e = 0 } );
            continue;
        }
        for (new_v, thickness) in graph.edges.entry(v).or_insert(Vec::new()) {
            *cur_energy.entry(*new_v).or_insert(0) += *thickness * cur_energy.get(&v).unwrap();
            if cur_energy.get(new_v).unwrap() >= graph.vertices.get(new_v).unwrap() {
                queue.push(*new_v, Reverse(*cur_energy.get(new_v).unwrap()));
            }
        }
    }
    let last_plant = graph.last_plant;
    let energy = *cur_energy.get(&last_plant).unwrap();
    if energy >= *graph.vertices.get(&last_plant).unwrap() {
        return energy;
    }
    return 0;
}

pub fn part1(input: String) -> i32 {
    let (mut graph, _) = parse_input(input);
    return activate(0, &mut graph, Option::None);
}

pub fn part2(input: String) -> i64 {
    let (mut graph, test_cases) = parse_input(input);
    let mut ans: i64 = 0;
    for test in test_cases {
        let energy = activate(0, &mut graph, Some(&test));
        println!("{}", energy);
        ans += energy as i64;
    }
    return ans;
}

mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(String::from("Plant 1 with thickness 1:
- free branch with thickness 1

Plant 2 with thickness 1:
- free branch with thickness 1

Plant 3 with thickness 1:
- free branch with thickness 1

Plant 4 with thickness 17:
- branch to Plant 1 with thickness 15
- branch to Plant 2 with thickness 3

Plant 5 with thickness 24:
- branch to Plant 2 with thickness 11
- branch to Plant 3 with thickness 13

Plant 6 with thickness 15:
- branch to Plant 3 with thickness 14

Plant 7 with thickness 10:
- branch to Plant 4 with thickness 15
- branch to Plant 5 with thickness 21
- branch to Plant 6 with thickness 34")), 774);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(String::from("Plant 1 with thickness 1:
- free branch with thickness 1

Plant 2 with thickness 1:
- free branch with thickness 1

Plant 3 with thickness 1:
- free branch with thickness 1

Plant 4 with thickness 10:
- branch to Plant 1 with thickness -25
- branch to Plant 2 with thickness 17
- branch to Plant 3 with thickness 12

Plant 5 with thickness 14:
- branch to Plant 1 with thickness 14
- branch to Plant 2 with thickness -26
- branch to Plant 3 with thickness 15

Plant 6 with thickness 150:
- branch to Plant 4 with thickness 5
- branch to Plant 5 with thickness 6


1 0 1
0 0 1
0 1 1")), 324);
    }
}