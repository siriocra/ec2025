use regex::Regex;

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
}

fn quality(sword: Vec<Bone>) -> String {
    let mut qual: String = String::new();
    for bone in sword {
        qual += bone.center.to_string().as_str();
    }
    return qual;
}

pub fn part1(input:String) -> String {
    let (_, numbers) = parse_input(input);
    let mut spine: Vec<Bone> = Vec::new();
    'outer: for number in numbers {
        for bone in &mut spine {
            if bone.left == None && number < bone.center {
                bone.left = Some(number);
                continue 'outer;
            } else if bone.right == None && number > bone.center {
                bone.right = Some(number);
                continue 'outer;
            }
        }
        let fishbone = Bone::new(number);
        spine.push(fishbone);
    }
    return quality(spine);
}

mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(String::from("58:5,3,7,8,9,10,4,5,7,8,8")), String::from("581078"));
    }
}
