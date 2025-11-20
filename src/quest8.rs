use std::cmp::min;
use std::cmp::max;

fn center(seq: Vec<i32>, nails: i32) -> i32 {
    let mut ans = 0;
    for i in 0..seq.len()-1 {
        if (seq[i] - seq[i+1]).abs() == nails / 2 {
            ans += 1;
        }
    }
    return ans;
}

fn crosses(a1: i32, a2: i32, b1: i32, b2: i32) -> bool {
    return 
        (min(a1, a2) < b1 && b1 < max(a1, a2) &&
        (b2 < min(a1, a2) || b2 > max(a1, a2))) ||
        (min(a1, a2) < b2 && b2 < max(a1, a2) &&
        (b1 < min(a1, a2) || b1 > max(a1, a2)))
    ;
}

fn intesections(seq: Vec<i32>, _: i32) -> i32 {
    let mut ans = 0;
    for i in 1..seq.len()-1 {
        for j in 0..i {
            if crosses(seq[i], seq[i+1], seq[j], seq[j+1]) {
                ans += 1;
            }
        }
    }
    return ans;
}

pub fn part1(input: String) -> i32 {
    let seq = input.split(',').map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>();
    return center(seq, 32);
}

pub fn part2(input: String) -> i32 {
    let seq = input.split(',').map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>();
    return intesections(seq, 256);
}

fn best_cut(seq: Vec<i32>, nails: i32) -> i32 {
    let mut ans = 0;
    for n1 in 1..=nails {
        for n2 in 1..=nails {
            if n1 == n2 {
                continue;
            }
            let mut cur = 0;
            for i in 0..seq.len()-1 {
                if min(n1, n2) == min(seq[i], seq[i+1]) &&
                   max(n1, n2) == max(seq[i], seq[i+1]) {
                    cur += 1;
                }
                if crosses(n1, n2, seq[i], seq[i+1]) {
                    cur += 1;
                }
            }
            ans = max(ans, cur);
        }
    }
    return ans;
}

pub fn part3(input: String) -> i32 {
    let seq = input.split(',').map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>();
    return best_cut(seq, 256);
}

mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(center(vec![1,5,2,6,8,4,1,7,3], 8), 4);
    }

    #[test]
    fn test_part2() {
        assert_eq!(intesections(vec![1,5,2,6,8,4,1,7,3,5,7,8,2], 8), 21);
    }

    #[test]
    fn test_part3() {
        assert_eq!(best_cut(vec![1,5,2,6,8,4,1,7,3,6], 8), 7);
    }
}