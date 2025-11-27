pub fn part1(input: String) -> i32 {
    let numbers = input.split(',').map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>();
    let columns = 91;
    let mut count = vec![0; columns];
    for num in numbers {
        let mut i = 1;
        while i * num < columns as i32 {
            count[(i * num) as usize] += 1;
            i += 1;
        }
    }
    let mut ans = 0;
    for i in 0..columns {
        ans += count[i];
    }
    return ans;
}

pub fn part2(input: String) -> i64 {
    let mut count = input.split(',').map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>();
    let columns = count.len();
    let mut ans: i64 = 1;
    'num: for num in 1..columns {
        let mut i = 1;
        while i * num < columns {
            if count[i * num - 1] == 0 {
                continue 'num;
            }
            i += 1;
        }
        let mut i = 1;
        while i * num < columns {
            count[i * num - 1] -= 1;
            i += 1;
        }
        ans *= num as i64;
    }
    return ans;
}

fn build_wall(numbers: &Vec<i32>, length: i64) -> i64 {
    let mut blocks = 0;
    for num in numbers {
        blocks += length / *num as i64;
    }
    return blocks;
}

fn binary_search(numbers: &Vec<i32>, blocks: i64) -> i64 {
    let mut l: i64 = 0;
    let mut r: i64 = 202_520_252_025_000;
    while l + 1 < r {
        let m = (l + r) / 2;
        if build_wall(numbers, m) <= blocks {
            l = m;
        } else {
            r = m;
        }
    }
    return l;
}

pub fn part3(input: String) -> i64 {
    let mut count = input.split(',').map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>();
    let columns = count.len();
    let mut distances = Vec::new();
    'num: for num in 1..columns {
        let mut i = 1;
        while i * num < columns {
            if count[i * num - 1] == 0 {
                continue 'num;
            }
            i += 1;
        }
        let mut i = 1;
        while i * num < columns {
            count[i * num - 1] -= 1;
            i += 1;
        }
        distances.push(num as i32);
    }
    return binary_search(&distances, 202520252025000);
}

mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(String::from("1,2,3,5,9")), 193);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(String::from("1,2,2,2,2,3,1,2,3,3,1,3,1,2,3,2,1,4,1,3,2,2,1,3,2,2")), 270);
    }

    #[test]
    fn test_part3() {
        assert_eq!(part3(String::from("1,2,2,2,2,3,1,2,3,3,1,3,1,2,3,2,1,4,1,3,2,2,1,3,2,2")), 94439495762954);
    }
}