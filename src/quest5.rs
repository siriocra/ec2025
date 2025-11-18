use regex::Regex;
use std::cmp::Ordering;
use std::cmp::max;
use std::cmp::min;

fn parse_input(input:String) -> (i32, Vec<i32>) {
    let re = Regex::new(r"(\d+):([0-9,]+)$").unwrap();
    let captures = re.captures(&input).unwrap();
    let numbers_str = captures[2].split(",").collect::<Vec<_>>();
    let numbers = numbers_str.into_iter().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>();
    return (
        captures[1].parse::<i32>().unwrap(), 
        numbers,
    )
}

struct Bone {
    center: i32,
    left: Option<i32>,
    right: Option<i32>,
}
impl Bone {
    fn new(a: i32) -> Bone {
        Bone {center: a, left: None, right: None}
    }
    fn number(&self) -> i32 {
        let mut num_str: String = String::from("");
        if self.left != None {
            num_str = self.left.unwrap().to_string();
        }
        num_str += self.center.to_string().as_str();
        if self.right != None {
            num_str += self.right.unwrap().to_string().as_str();
        }
        return num_str.parse().unwrap();
    }
}

fn quality(sword: &Vec<Bone>) -> String {
    let mut qual: String = String::new();
    for bone in sword {
        qual += bone.center.to_string().as_str();
    }
    return qual;
}

fn create_sword(numbers: &Vec<i32>) -> Vec<Bone> {
    let mut spine: Vec<Bone> = Vec::new();
    'outer: for number in numbers {
        for bone in &mut spine {
            if bone.left == None && *number < bone.center {
                bone.left = Some(*number);
                continue 'outer;
            } else if bone.right == None && *number > bone.center {
                bone.right = Some(*number);
                continue 'outer;
            }
        }
        let fishbone = Bone::new(*number);
        spine.push(fishbone);
    }
    return spine;
}

fn compare_swords<'a, 'b> (a: &'a (i32, &Vec<Bone>), b: &'b (i32, &Vec<Bone>)) -> Ordering {
    let q_a:i64 = quality(a.1).parse().unwrap();
    let q_b:i64 = quality(b.1).parse().unwrap();
    if q_a < q_b {
        return Ordering::Greater;
    } else if q_a > q_b {
        return Ordering::Less;
    } else {
        // Sword qualities are equal
        for i in 0..min(a.1.len(), b.1.len()) {
            if a.1[i].number() < b.1[i].number() {
                return Ordering::Greater;
            } else if a.1[i].number() > b.1[i].number() {
                return Ordering::Less;
            }
        }
        if a.1.len() < b.1.len() {
            return Ordering::Greater;
        } else if a.1.len() > b.1.len() {
            return Ordering::Less;
        }
        if a.0 < b.0 {
            return Ordering::Greater;
        } else if a.0 > b.0 {
            return Ordering::Less;
        }
    }
    return Ordering::Equal;
}

pub fn part1(input:String) -> String {
    let (_, numbers) = parse_input(input);
    let sword = create_sword(&numbers);
    return quality(&sword);
}

pub fn part2(input:String) -> i64 {
    let mut max_qual = 0;
    let mut min_qual = 1_000_000_000_000;
    for line in input.lines() {
        let (_, numbers) = parse_input(line.to_string());
        let sword = create_sword(&numbers);
        let qual = quality(&sword);
        let qual_int = qual.parse::<i64>().unwrap();
        max_qual = max(max_qual, qual_int);
        min_qual = min(min_qual, qual_int);
    }
    return max_qual - min_qual;
}

pub fn part3(input:String) -> i32 {
    let mut swords: Vec<(i32, Vec<Bone>)> = Vec::new();
    for line in input.lines() {
        let (id, numbers) = parse_input(line.to_string());
        let sword = create_sword(&numbers);
        swords.push((id, sword));
    }
    swords.sort_by(|a: &(i32, Vec<Bone>), b: &(i32, Vec<Bone>)| 
        compare_swords(&(a.0, &a.1), &(b.0, &b.1)));
    let mut checksum = 0;
    for i in 0..swords.len() {
        checksum += swords[i].0 * (i as i32 + 1);
    }
    return checksum;
}

mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(String::from("58:5,3,7,8,9,10,4,5,7,8,8")), String::from("581078"));
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(String::from("1:2,4,1,1,8,2,7,9,8,6
2:7,9,9,3,8,3,8,8,6,8
3:4,7,6,9,1,8,3,7,2,2
4:6,4,2,1,7,4,5,5,5,8
5:2,9,3,8,3,9,5,2,1,4
6:2,4,9,6,7,4,1,7,6,8
7:2,3,7,6,2,2,4,1,4,2
8:5,1,5,6,8,3,1,8,3,9
9:5,7,7,3,7,2,3,8,6,7
10:4,1,9,3,8,5,4,3,5,5")), 77053);
    }

    #[test]
    fn test_part3() {
        assert_eq!(part3(String::from("1:7,1,9,1,6,9,8,3,7,2
2:7,1,9,1,6,9,8,3,7,2")), 4);
        assert_eq!(part3(String::from("1:7,1,9,1,6,9,8,3,7,2
2:6,1,9,2,9,8,8,4,3,1
3:7,1,9,1,6,9,8,3,8,3
4:6,1,9,2,8,8,8,4,3,1
5:7,1,9,1,6,9,8,3,7,3
6:6,1,9,2,8,8,8,4,3,5
7:3,7,2,2,7,4,4,6,3,1
8:3,7,2,2,7,4,4,6,3,7
9:3,7,2,2,7,4,1,6,3,7")), 260);
    }

    #[test]
    fn test_bone_number() {
        assert_eq!(Bone{left: Some(3), center: 5, right: Some(7)}.number(), 357);
    }
}
