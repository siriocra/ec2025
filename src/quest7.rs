use std::{clone, collections::HashMap};
use regex::Regex;

fn parse_input(input: &String) -> (Vec<&str>, HashMap<char, Vec<char>>) {
    let lines = input.lines().collect::<Vec<_>>();
    let names = lines[0].split(',').collect::<Vec<_>>();
    let mut graph = HashMap::new();
    for i in 2..lines.len() {
        let re = Regex::new(r"([A-Za-z]) > ([A-Za-z,]+)$").unwrap();
        let captures = re.captures(&lines[i]).unwrap();
        let c = captures[1].parse::<char>().unwrap();
        let edges = captures[2].split(',').map(|x| x.parse::<char>().unwrap()).collect::<Vec<_>>();
        graph.insert(c, edges);
    }
    return (names, graph);
}

fn check_name(name_vec: &Vec<char>, graph: &mut HashMap<char, Vec<char>>) -> bool{
    'characters: for i in 0..name_vec.len()-1 {
        let possible_next = graph.entry(name_vec[i]).or_insert(Vec::new());
        for next in possible_next {
            if *next == name_vec[i+1] {
                continue 'characters;
            }
        }
        return false;
    }
    return true;
}

pub fn part1(input: String) -> String {
    let (names, mut graph) = parse_input(&input);
    for name in names {
        let name_vec = name.chars().collect::<Vec<_>>();
        if check_name(&name_vec, &mut graph) {
            return name.to_string();
        }        
    }
    return String::new();
}

pub fn part2(input: String) -> i32 {
    let (names, mut graph) = parse_input(&input);
    let mut ans: i32 = 0;
    for (index, name) in names.iter().enumerate() {
        let name_vec = name.chars().collect::<Vec<_>>();
        if check_name(&name_vec, &mut graph) {
            ans += (index + 1) as i32;
        }
    }
    return ans;
}

fn calc_names(
    prefix: &mut String, 
    last: char, 
    graph: &HashMap<char, Vec<char>>,
    count: &mut HashMap<String, i32>,
) -> i32 {
    if count.contains_key(prefix.as_str()) {
        return *count.get(prefix.as_str()).unwrap();
    }
    if prefix.len() == 11 {
        return 1;
    }
    let mut ans = 0;
    if prefix.len() >= 7 {
        ans += 1;
    }
    let next = graph.get(&last);
    if next == None {
        count.insert(prefix.clone(), ans);
        return ans;
    }
    for c in next.unwrap() {
        prefix.push(*c);
        ans += calc_names(prefix, *c, graph, count);
        prefix.pop();
    }
    count.insert(prefix.clone(), ans);
    return ans;
}

pub fn part3(input: String) -> i32 {
    let (prefixes, mut graph) = parse_input(&input);
    let mut filtered_prefixes = Vec::new();
    'p1: for p1 in &prefixes {
        for p2 in &prefixes {
            if (*p1).starts_with(*p2) && p1 != p2 {
                continue 'p1;
            }
        }
        let name_vec = p1.chars().collect::<Vec<_>>();
        if check_name(&name_vec, &mut graph) {
            filtered_prefixes.push(*p1);
        }
    }
    let mut ans = 0;
    for prefix in filtered_prefixes {
        let mut name = String::from(prefix);
        let last_char = name.chars().last().unwrap();
        let mut count: HashMap<String, i32> = HashMap::new();
        ans += calc_names(&mut name, last_char, &graph, &mut count);
    }
    return ans;
}

mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(String::from("Oronris,Urakris,Oroneth,Uraketh

r > a,i,o
i > p,w
n > e,r
o > n,m
k > f,r
a > k
U > r
e > t
O > r
t > h")), "Oroneth");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(String::from("Xanverax,Khargyth,Nexzeth,Helther,Braerex,Tirgryph,Kharverax

r > v,e,a,g,y
a > e,v,x,r
e > r,x,v,t
h > a,e,v
g > r,y
y > p,t
i > v,r
K > h
v > e
B > r
t > h
N > e
p > h
H > e
l > t
z > e
X > a
n > v
x > z
T > i")), 23);
    }

    #[test]
    fn test_part3() {
        assert_eq!(part3(String::from("Xaryt

X > a,o
a > r,t
r > y,e,a
h > a,e,v
t > h
v > e
y > p,t")), 25);
        assert_eq!(part3(String::from("Khara,Xaryt,Noxer,Kharax

r > v,e,a,g,y
a > e,v,x,r,g
e > r,x,v,t
h > a,e,v
g > r,y
y > p,t
i > v,r
K > h
v > e
B > r
t > h
N > e
p > h
H > e
l > t
z > e
X > a
n > v
x > z
T > i")), 1154);
    }
}