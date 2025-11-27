use std::cmp::max;

fn parse_input(input: String) -> ((usize, usize), Vec<Vec<u32>>) {
    let mut numbers = Vec::new();
    let mut volcano = (0, 0);
    for line in input.lines() {
        if line.contains('@') {
            numbers.push(line.replace('@', "0").chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<_>>());
            for j in 0..numbers[0].len()-1 {
                if numbers[numbers.len()-1][j] == 0 {
                    volcano = (numbers.len() - 1, j);
                }
            }
        } else {
            numbers.push(line.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<_>>());
        }
    }
    return (volcano, numbers);
}

fn inside(point: &(usize, usize), volcano: &(usize, usize), radius: i32) -> bool {
    return (volcano.0 as i32 - point.0 as i32) * (volcano.0 as i32 - point.0 as i32) as i32 + (volcano.1 as i32 - point.1 as i32) * (volcano.1 as i32 - point.1 as i32) <= radius * radius;
}

pub fn part1(input: String) -> i32 {
    let (volcano, numbers) = parse_input(input);
    println!("volcano {} {}", volcano.0, volcano.1);
    let radius = 10;
    let mut ans: i32 = 0;
    for i in 0..numbers.len() {
        for j in 0..numbers[0].len() {
            if inside(&(i, j), &volcano, radius) {
                ans += numbers[i][j] as i32;
            }
        }
    }
    return ans;
}

pub fn part2(input: String) -> i32 {
    let (volcano, numbers) = parse_input(input);
    let radius = max(volcano.0, volcano.1);
    let mut max_destruction = 0;
    let mut max_desturction_r = 0;
    for r in 1i32..radius as i32 {
        let mut ans: i32 = 0;
        for i in 0..numbers.len() {
            for j in 0..numbers[0].len() {
                if inside(&(i, j), &volcano, r)
                    && !inside(&(i, j), &volcano, r - 1){
                    ans += numbers[i][j] as i32;
                }
            }
        }
        if max_destruction < ans {
            max_destruction = ans;
            max_desturction_r = r;
        }
    }
    return max_destruction * max_desturction_r;
}

mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(String::from("189482189843433862719
279415473483436249988
432746714658787816631
428219317375373724944
938163982835287292238
627369424372196193484
539825864246487765271
517475755641128575965
685934212385479112825
815992793826881115341
1737798467@7983146242
867597735651751839244
868364647534879928345
519348954366296559425
134425275832833829382
764324337429656245499
654662236199275446914
317179356373398118618
542673939694417586329
987342622289291613318
971977649141188759131")), 1573);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(String::from("4547488458944
9786999467759
6969499575989
7775645848998
6659696497857
5569777444746
968586@767979
6476956899989
5659745697598
6874989897744
6479994574886
6694118785585
9568991647449")), 1090);
    }
}