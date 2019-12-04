use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut total = 0;

    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let mut mass = (ip.parse::<i32>().unwrap() / 3) - 2;
                while mass > 0 {
                    total += mass;
                    mass = (mass / 3) - 2;
                }
            }
        }
    }

    println!("{}", total); // 5070541
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
