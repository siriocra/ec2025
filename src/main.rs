mod day1;
mod day2;
mod day3;
mod quest4;
mod quest5;
mod quest6;
mod quest7;
mod quest8;
mod quest9;
mod quest10;
mod quest11;
mod quest12;
mod quest13;
mod quest14;
mod quest15;
mod quest16;
mod quest17;
mod quest18;
mod quest19;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn read_file(filename:String) -> String {
    let path_str = "input/".to_owned() + &filename;
    let path = Path::new(&path_str);
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => println!("{}", path_str),
    }
    s
}

fn main() {
    let q19_file = "everybody_codes_e2025_q19_p3.txt";
    let input = read_file(q19_file.to_string());
    println!("{}", quest19::part2(input));
}
