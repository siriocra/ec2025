mod day1;
mod day2;
mod day3;
mod quest4;
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
    /*let test_file = "test.txt";
    let d1_file = "everybody_codes_e2025_q01_p3.txt";
    let input = read_file(d1_file.to_string());
    day1::part3(input);
    let d2_file = "everybody_codes_e2025_q02_p3.txt";
    let input = read_file(d2_file.to_string());
    day2::part3(input);
    let d3_file = "everybody_codes_e2025_q03_p3.txt";
    let input = read_file(d3_file.to_string());
    println!("{}", day3::part3(input));*/
    let q4_file = "everybody_codes_e2025_q04_p3.txt";
    let input = read_file(q4_file.to_string());
    println!("{}", quest4::part3(input));
}
