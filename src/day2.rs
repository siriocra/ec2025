use regex::Regex;

 #[derive(Clone)]
 struct Number {
    x:i64,
    y:i64,
}
impl Number {
    fn mul(&self, num:&Number) -> Number {
        Number {
            x: self.x * num.x - self.y * num.y,
            y: self.x * num.y + self.y * num.x,
        }
    }
    fn add(&self, num:&Number) -> Number {
        Number { x: (self.x + num.x), y: (self.y + num.y) }
    }
    fn div(&self, num:&Number) -> Number {
        Number { x: (self.x / num.x), y: (self.y / num.y) }
    }
}

fn parse_input(input:String) -> Number {
    let re = Regex::new(r"A=\[(-?\d+),(-?\d+)\]$").unwrap();
    let captures = re.captures(&input).unwrap();
    let num = Number{
        x:captures[1].parse::<i64>().unwrap(), 
        y:captures[2].parse::<i64>().unwrap(),
    };
    num
}

const MAX_VALUE:i64 = 1_000_000;

fn calculate(p:&Number, max_count:i32, divider:&Number) -> Option<Number>{
    let mut counter = max_count;
    let mut result = Number{x:0, y:0};
    while counter > 0 {
        result = result.mul(&result);
        result = result.div(divider);
        result = result.add(p);
        if result.x.abs() > MAX_VALUE || result.y.abs() > MAX_VALUE {
            return None;
        }
        counter -= 1;
    }
    
    return Some(result)
}

pub fn part1(input:String) {
    let a = parse_input(input);
    let divider = Number{x: 10, y: 10};
    let result = calculate(&a,3, &divider).unwrap();
    println!("[{},{}]", result.x, result.y);
}


fn is_engraved(p:&Number) -> bool {
    let max_count = 100;
    let divider = &Number { x: (100_000), y: (100_000) };
    let result = calculate(p, max_count, divider);
    return result.is_some()
}

pub fn part2(input:String) {
    let a = parse_input(input);
    let opa = a.add(&Number{x:1000, y:1000});
    let mut total = 0;
    for x in (a.x..=opa.x).step_by(10) {
        for y in (a.y..=opa.y).step_by(10) {
            if is_engraved(&Number { x, y }) {
                total += 1;
            }
        }
    }
    println!("{}", total)
}

pub fn part3(input:String) {
    let a = parse_input(input);
    let opa = a.add(&Number{x:1000, y:1000});
    let mut total = 0;
    for x in a.x..=opa.x {
        for y in a.y..=opa.y {
            if is_engraved(&Number { x, y }) {
                total += 1;
            }
        }
    }
    println!("{}", total)
}

mod tests {
    use super::*;

    #[test]
    fn test_calculate() {
        let divider = &Number { x: (100_000), y: (100_000) };
        let t1 = calculate(&Number{x:35630,y:-64880}, 100, divider).unwrap();
        assert_eq!(t1.x, -2520);
        assert_eq!(t1.y, -5355);
    }
}