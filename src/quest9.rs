use regex::Regex;
use dsu_tree::DsuRoot;
use std::cmp::min;
use std::cmp::max;

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

pub fn part3(input: String) -> i32 {
    let data = input.lines().map(|line| parse_line(line)).collect::<Vec<_>>();
    let mut roots: Vec<DsuRoot<_>> = Vec::new();
    for i in 0..data.len() {
        roots.push(DsuRoot::new(i+1));
    }
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
                    let (dsu_1, dsu_2) = roots.split_at_mut(i);
                    dsu_1[j].merge_into(&mut dsu_2[0]);
                    let (dsu_1, dsu_2) = roots.split_at_mut(max(i, k));
                    dsu_1[min(i, k)].merge_into(&mut dsu_2[0]);
                    let (dsu_1, dsu_2) = roots.split_at_mut(max(j, k));
                    dsu_1[min(j, k)].merge_into(&mut dsu_2[0]);
                }
            }
        }
    }
    let mut ans = 0;
    let mut max_family = 0;
    for i in 0..data.len() {
        let mut family = 1;
        let mut cur_ans = data[i].0;
        for j in i+1..data.len() {
            let (dsu_1, dsu_2) = roots.split_at_mut(j);
            if DsuRoot::same(&mut dsu_1[i], &mut dsu_2[0]) {
                family += 1;
                cur_ans += data[j].0;
            }
        }
        if max_family < family {
            max_family = family;
            ans = cur_ans;
        }
    }
    return ans as i32;
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

    #[test]
    fn test_part3() {
        assert_eq!(part3(String::from("1:GCAGGCGAGTATGATACCCGGCTAGCCACCCC
2:TCTCGCGAGGATATTACTGGGCCAGACCCCCC
3:GGTGGAACATTCGAAAGTTGCATAGGGTGGTG
4:GCTCGCGAGTATATTACCGAACCAGCCCCTCA
5:GCAGCTTAGTATGACCGCCAAATCGCGACTCA
6:AGTGGAACCTTGGATAGTCTCATATAGCGGCA
7:GGCGTAATAATCGGATGCTGCAGAGGCTGCTG")), 12);
        assert_eq!(part3(String::from("1:GCAGGCGAGTATGATACCCGGCTAGCCACCCC
2:TCTCGCGAGGATATTACTGGGCCAGACCCCCC
3:GGTGGAACATTCGAAAGTTGCATAGGGTGGTG
4:GCTCGCGAGTATATTACCGAACCAGCCCCTCA
5:GCAGCTTAGTATGACCGCCAAATCGCGACTCA
6:AGTGGAACCTTGGATAGTCTCATATAGCGGCA
7:GGCGTAATAATCGGATGCTGCAGAGGCTGCTG
8:GGCGTAAAGTATGGATGCTGGCTAGGCACCCG")), 36);
    }
}
