use std::cmp::{Reverse, max};

use array2d::Array2D;
use priority_queue::PriorityQueue;

fn parse_input(input: String) -> ((usize, usize), Vec<Vec<u32>>, (usize, usize)) {
    let mut numbers = Vec::new();
    let mut volcano = (0, 0);
    let mut start = (0, 0);
    for line in input.lines() {
        if line.contains('@') {
            numbers.push(line.replace('@', "0").chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<_>>());
            for j in 0..numbers[0].len()-1 {
                if numbers[numbers.len()-1][j] == 0 {
                    volcano = (numbers.len() - 1, j);
                }
            }
        } else if line.contains('S') {
            numbers.push(line.replace('S', "0").chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<_>>());
            for j in 0..numbers[0].len()-1 {
                if numbers[numbers.len()-1][j] == 0 {
                    start = (numbers.len() - 1, j);
                }
            }
        } else {
            numbers.push(line.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<_>>());
        }
    }
    return (volcano, numbers, start);
}

fn inside(point: &(usize, usize), volcano: &(usize, usize), radius: i32) -> bool {
    return (volcano.0 as i32 - point.0 as i32) * (volcano.0 as i32 - point.0 as i32) as i32 + (volcano.1 as i32 - point.1 as i32) * (volcano.1 as i32 - point.1 as i32) <= radius * radius;
}

pub fn part1(input: String) -> i32 {
    let (volcano, numbers, _) = parse_input(input);
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
    let (volcano, numbers, _) = parse_input(input);
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

fn calc_distance(radius: i32, volcano: &(usize, usize), start: &(usize, usize), numbers: &Vec<Vec<u32>>) -> (Array2D<u32>, Array2D<char>) {
    let mut queue = PriorityQueue::new();
    let (h, w) = (numbers.len(), numbers[0].len());
    let mut distance = Array2D::filled_with(1_000_000, h, w);
    let mut path = Array2D::filled_with('.', h, w);
    distance[*start] = 0;
    let neighbors: [(i32, i32); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];

    queue.push(*start, Reverse(0));
    while !queue.is_empty() {
        let (p, p_dist) = queue.pop().unwrap();
        if p.0 == volcano.0 {
            if p.1 < volcano.1 {
                path[p] = 'L';
            } else {
                path[p] = 'R';
            }
        }
        for (dx, dy) in neighbors {
            if 0i32 > (p.0 as i32 + dx) || (p.0 as i32 + dx) >= h as i32 
                || 0i32 > (p.1 as i32 + dy) || (p.1 as i32 + dy) >= w as i32 {
                    continue;
            }
            let new_p = ((p.0 as i32 + dx) as usize, (p.1 as i32 + dy) as usize);
            if inside(&new_p, &volcano, radius) {
                continue;
            }
            if distance[new_p] > distance[p] + numbers[new_p.0][new_p.1] {
                distance[new_p] = distance[p] + numbers[new_p.0][new_p.1];
                path[new_p] = path[p];
                if queue.contains(&new_p) {
                    queue.change_priority(&new_p, Reverse(distance[new_p]));
                } else {
                    queue.push(new_p, Reverse(distance[new_p]));
                }
            }
        }
    }
    return (distance, path);
}

fn is_surrounded(radius: i32, volcano: &(usize, usize), start: &(usize, usize), numbers: &Vec<Vec<u32>>) -> (bool, i32) {
    let available_time = (radius + 1) as u32 * 30;
    let (distance, path) = calc_distance(radius, volcano, start, numbers);
    let mut time = 1000000;
    let neighbors: [(i32, i32); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];
    for i in 1..numbers.len()-1 {
        for j in 1..numbers[0].len()-1 {
            if path[(i, j)] == 'L' {
                for (dx, dy) in neighbors {
                    let new_coords = ((i as i32 + dx) as usize, (j as i32 + dy) as usize);
                    if path[new_coords] == 'R' {
                        if time > distance[(i, j)] + distance[new_coords] {
                            time = distance[(i, j)] + distance[new_coords];
                        }
                    }
                }
            }
        }
    }
    return (time < available_time, time as i32);
}

pub fn part3(input: String) -> i32 {
    let (volcano, numbers, start) = parse_input(input);
    let radius = max(volcano.0, volcano.1);
    let mut min_time = 1000000;
    let mut min_time_r = 0;
    for r in 1i32..radius as i32 {
        let (success, time) = is_surrounded(r, &volcano, &start, &numbers);
        if success && min_time > time {
            min_time = time;
            min_time_r = r;
            break;
        }
    }
    println!("{}", min_time_r);
    return min_time * min_time_r;
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

    #[test]
    fn test_part3() {
        assert_eq!(part3(String::from("2645233S5466644
634566343252465
353336645243246
233343552544555
225243326235365
536334634462246
666344656233244
6426432@2366453
364346442652235
253652463426433
426666225623563
555462553462364
346225464436334
643362324542432
463332353552464")), 592);
        assert_eq!(part3(String::from("545233443422255434324
5222533434S2322342222
523444354223232542432
553522225435232255242
232343243532432452524
245245322252324442542
252533232225244224355
523533554454232553332
522332223232242523223
524523432425432244432
3532242243@4323422334
542524223994422443222
252343244322522222332
253355425454255523242
344324325233443552555
423523225325255345522
244333345244325322335
242244352245522323422
443332352222535334325
323532222353523253542
553545434425235223552")), 330);
        assert_eq!(part3(String::from("5441525241225111112253553251553
133522122534119S911411222155114
3445445533355599933443455544333
3345333555434334535435433335533
5353333345335554434535533555354
3533533435355443543433453355553
3553353435335554334453355435433
5435355533533355533535335345335
4353545353545354555534334453353
4454543553533544443353355553453
5334554534533355333355543533454
4433333345445354553533554555533
5554454343455334355445533453453
4435554534445553335434455334353
3533435453433535345355533545555
534433533533535@353533355553345
4453545555435334544453344455554
4353333535535354535353353535355
4345444453554554535355345343354
3534544535533355333333445433555
3535333335335334333534553543535
5433355333553344355555344553435
5355535355535334555435534555344
3355433335553553535334544544333
3554333535553335343555345553535
3554433545353554334554345343343
5533353435533535333355343333555
5355555353355553535354333535355
4344534353535455333455353335333
5444333535533453535335454535553
3534343355355355553543545553345")), 3180);
    }
}