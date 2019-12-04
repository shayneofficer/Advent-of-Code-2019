use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let path = Path::new("./input.txt");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why.description()),
        Ok(file) => file,
    };

    let mut input = String::new();
    match file.read_to_string(&mut input) {
        Err(why) => panic!("couldn't read {}: {}", display, why.description()),
        Ok(_) => {
            let string_intcode: Vec<&str> = input.trim().split(',').collect();
            
            let mut intcode = Vec::new(); 
            for s in string_intcode {
                intcode.push(s.parse::<i32>().unwrap());
            }

            for noun in 0..99 {
                for verb in 0..99 {
                    let output = process_intcode(intcode.clone(), noun, verb);
                    if output == 19690720 {
                        println!("Noun: {}, Verb: {}", noun, verb); // Noun: 95, Verb: 7
                        break;
                    }
                }
            }
        }
    }
}

fn process_intcode(mut intcode: Vec<i32>, noun: i32, verb: i32) -> i32 {
    intcode[1] = noun;
    intcode[2] = verb;
    
    for i in (0..(intcode.len() - 1)).step_by(4) {
        let opcode = intcode[i];
    
        if opcode == 99 {
            break;
        }
    
        let operand_one = intcode[intcode[i + 1] as usize];
        let operand_two = intcode[intcode[i + 2] as usize];
    
        let result = match opcode {
            1 => operand_one + operand_two,
            2 => operand_one * operand_two,
            _ => -1
        };
    
        let result_position = intcode[i + 3] as usize;
        intcode[result_position] = result;
    }
    
    return intcode[0];
}
