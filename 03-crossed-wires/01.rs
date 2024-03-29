use std::fs;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    
    let input = fs::read_to_string("./input.txt")
        .expect("Something went wrong");
    
    let wires: Vec<&str> = input.split('\n').collect();
    
    let wire_one: Vec<&str> = wires[0].trim().split(',').collect();
    let wire_two: Vec<&str> = wires[1].trim().split(',').collect();
    
    let mut steps: Vec<(i64, i64)> = Vec::new();
    let mut intersections: Vec<(i64, i64)> = Vec::new();
    // (-994, 210), (-722, 274), (-290, -1036), (-277, -1036), (529, -582), (-3363, -4440)
    // (-3363, -3937), (-3109, -3139), (-2618, -3077), (-2538, -3077), (-2523, -2959)
    // (-2523, -2820), (-2511, -2011), (-2355, -2011), (-2208, -1811), (-2208, -1728)
    // (-2208, -1582), (-2208, -1523), (-2167, -1012), (-2157, -1012), (-1690, -1012)
    // (-783, -433), (-596, -1036), (752, 889), (531, 712), (855, 41), (991, 41)
    // (1197, -15), (1197, -314), (1617, -314), (1435, -197), (991, 592), (855, 592)
    
    let mut current_position: (i64, i64) = (0, 0);
    
    for instruction in wire_one {
        let (direction, distance) = instruction.split_at(1);
        let dist: u32 = distance.parse::<u32>().unwrap();
        for _i in 0..dist {
            match direction {
                "U" => current_position.1 += 1,
                "D" => current_position.1 -= 1,
                "L" => current_position.0 -= 1,
                "R" => current_position.0 += 1,
                _ => println!("Error")
            }
            if !steps.contains(&current_position) {
                steps.push(current_position);
            }
        }
    }
                    
    current_position = (0, 0);
                    
    for instruction in wire_two {
        let (direction, distance) = instruction.split_at(1);
        let dist: u32 = distance.parse::<u32>().unwrap();
        for _i in 0..dist {
            match direction {
                "U" => current_position.1 += 1,
                "D" => current_position.1 -= 1,
                "L" => current_position.0 -= 1,
                "R" => current_position.0 += 1,
                _ => println!("Error")
            }
            if steps.contains(&current_position) {
                intersections.push(current_position);
            }
        }
    }
                                    
    let mut v = intersections.iter().map(|&point| (point.0 as i64).abs() + (point.1 as i64).abs())
        .collect::<Vec<i64>>();
    v.sort();

    println!("{}", v[0]); // 896
    println!("{}", start.elapsed().as_secs()); // 897
}
