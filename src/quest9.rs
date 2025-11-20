use regex::Regex;

fn parse_line(line: &str) -> (i32, Vec<char>) {
    let re = Regex::new(r"(\d+):([CAGT]+)$").unwrap();
    let captures = re.captures(&line).unwrap();
    return (
        captures[1].parse::<i32>().unwrap(),
        captures[2].chars().collect::<Vec<_>>(),
    );
}

fn compare_dna(dna1: &Vec<char>, dna2: &Vec<char>) -> i32 {
    let mut same = 0;
    for i in 0..dna1.len() {
        if dna1[i] == dna2[i] {
            same += 1;
        }
    }
    return same;
}

fn is_child(parent1: &Vec<char>, parent2: &Vec<char>, child: &Vec<char>) -> bool {
    for i in 0..child.len() {
        if child[i] != parent1[i] && child[i] != parent2[i] {
            return false;
        }
    }
    return true;
}

pub fn part1(input: String) -> i32 {
    let data = input.lines().map(|line| parse_line(line)).collect::<Vec<_>>();
    for i in 0..data.len() {
        for j in 0..i {
            for k in 0..data.len() {
                if k == i || k == j {
                    continue;
                }
                if is_child(
                    &data[i].1, 
                    &data[j].1, 
                    &data[k].1
                ) {
                    return compare_dna(&data[i].1, &data[k].1) * 
                        compare_dna(&data[j].1, &data[k].1);
                }
            }
        }
    }
    return 0;
}

pub fn part2(input: String) -> i32 {
    let data = input.lines().map(|line| parse_line(line)).collect::<Vec<_>>();
    let mut ans = 0;
    for i in 0..data.len() {
        for j in 0..i {
            for k in 0..data.len() {
                if k == i || k == j {
                    continue;
                }
                if is_child(
                    &data[i].1, 
                    &data[j].1, 
                    &data[k].1
                ) {
                    ans += compare_dna(&data[i].1, &data[k].1) * 
                        compare_dna(&data[j].1, &data[k].1);
                }
            }
        }
    }
    return ans;
}

mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(String::from("1:CAAGCGCTAAGTTCGCTGGATGTGTGCCCGCG
2:CTTGAATTGGGCCGTTTACCTGGTTTAACCAT
3:CTAGCGCTGAGCTGGCTGCCTGGTTGACCGCG")), 414);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(String::from("1:GCAGGCGAGTATGATACCCGGCTAGCCACCCC
2:TCTCGCGAGGATATTACTGGGCCAGACCCCCC
3:GGTGGAACATTCGAAAGTTGCATAGGGTGGTG
4:GCTCGCGAGTATATTACCGAACCAGCCCCTCA
5:GCAGCTTAGTATGACCGCCAAATCGCGACTCA
6:AGTGGAACCTTGGATAGTCTCATATAGCGGCA
7:GGCGTAATAATCGGATGCTGCAGAGGCTGCTG")), 1245);
    }
}
