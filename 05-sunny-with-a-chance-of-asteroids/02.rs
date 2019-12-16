use std::fs;
use std::io::{stdin,stdout,Write};


fn main() {
    let input = fs::read_to_string("./input.txt")
        .expect("Something went wrong");
    
    let intcode: Vec<i32> = input
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    process_intcode(intcode); // 12648139
}


fn process_intcode(mut intcode: Vec<i32>) {
    let mut curr = 0;
    while curr < intcode.len() {
        // This is very inelegant
        let mut opcode: Vec<i32> = vec![0; 5]; 
        opcode[0] = intcode[curr] % 10;
        opcode[1] = (intcode[curr] % 100) / 10;
        opcode[2] = (intcode[curr] % 1000) / 100;
        opcode[3] = (intcode[curr] % 10000) / 1000;
        opcode[4] = intcode[curr] / 10000;

        let instruction = (10 * opcode[1]) + opcode[0];
        let operation = match instruction {
            01 => "addition",
            02 => "multiplication",
            03 => "input",
            04 => "output",
            05 => "jump-if-true",
            06 => "jump-if-false",
            07 => "less-than",
            08 => "equals",
            99 => "halt",
            _ => "error"
        };

        if operation == "halt" || operation == "error" {
            break;
        }
        
        if operation == "addition" {
            let operand_one = if opcode[2] == 0 
                {intcode[intcode[curr + 1] as usize]}
                else {intcode[curr + 1]};
    
            let operand_two = if opcode[3] == 0 
                {intcode[intcode[curr + 2] as usize]}
                else {intcode[curr + 2]};

            let result_position = intcode[curr + 3] as usize;
            intcode[result_position] = operand_one + operand_two;
            curr += 4;
        }

        if operation == "multiplication" {
            let operand_one = if opcode[2] == 0 
                {intcode[intcode[curr + 1] as usize]}
                else {intcode[curr + 1]};

            let operand_two = if opcode[3] == 0 
                {intcode[intcode[curr + 2] as usize]}
                else {intcode[curr + 2]};

            let result_position = intcode[curr + 3] as usize;
            intcode[result_position] = operand_one * operand_two;
            curr += 4;
        }

        if operation == "input" {
            let result_position = intcode[curr + 1] as usize;
            
            let mut input_text = String::new();
            print!("Please enter an input value: ");
            let _ = stdout().flush();
            stdin()
                .read_line(&mut input_text)
                .expect("Did not enter a correct string");

            match input_text.trim().parse::<i32>() {
                Ok(input_value) => {
                    intcode[result_position] = input_value;
                    curr += 2;
                },
                Err(..) => {
                    println!("this was not an integer: {}", input_text);
                    break;
                },
            };
        }

        if operation == "output" {
            let result_position = intcode[curr + 1] as usize;
            println!("output: {}", intcode[result_position]);
            
            curr += 2;
        }

        if operation == "jump-if-true" {
            let operand_one = if opcode[2] == 0 
                {intcode[intcode[curr + 1] as usize]}
                else {intcode[curr + 1]};

            let operand_two = if opcode[3] == 0 
                {intcode[intcode[curr + 2] as usize]}
                else {intcode[curr + 2]}; 

            if operand_one != 0 {
                curr = operand_two as usize;
            } else {
                curr += 3;
            }
        }

        if operation == "jump-if-false" {
            let operand_one = if opcode[2] == 0 
                {intcode[intcode[curr + 1] as usize]}
                else {intcode[curr + 1]};

            let operand_two = if opcode[3] == 0 
                {intcode[intcode[curr + 2] as usize]}
                else {intcode[curr + 2]}; 

            if operand_one == 0 {
                curr = operand_two as usize;
            } else {
                curr += 3;
            }
        }

        if operation == "less-than" {
            let operand_one = if opcode[2] == 0 
                {intcode[intcode[curr + 1] as usize]}
                else {intcode[curr + 1]};

            let operand_two = if opcode[3] == 0 
                {intcode[intcode[curr + 2] as usize]}
                else {intcode[curr + 2]}; 

            let result_position = intcode[curr + 3] as usize;

            intcode[result_position] = if operand_one < operand_two
                {1}
                else {0};
            
            curr += 4;
        }

        if operation == "equals" {
            let operand_one = if opcode[2] == 0 
                {intcode[intcode[curr + 1] as usize]}
                else {intcode[curr + 1]};

            let operand_two = if opcode[3] == 0 
                {intcode[intcode[curr + 2] as usize]}
                else {intcode[curr + 2]}; 

            let result_position = intcode[curr + 3] as usize;

            intcode[result_position] = if operand_one == operand_two
                {1}
                else {0};

            curr += 4;
        }

    }
}
