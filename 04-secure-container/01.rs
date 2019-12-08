
fn main() {
    let input_max: u32 = 905157;
    let input_min: u32 = 372037;

    let mut total_possible_passwords: u32 = 0;

    for i in input_min..input_max {
        let sequence: Vec<u32> = number_to_vec(i);
        if is_non_decreasing(&sequence) && contains_pair(&sequence) {
            total_possible_passwords += 1;
        }
    }

    println!("{}", total_possible_passwords); // 481
}

fn is_non_decreasing(sequence: &Vec<u32>) -> bool {
    for i in 0..sequence.len() - 1 {
        if sequence[i] > sequence[i + 1] {
            return false;
        }
    }

    return true;
}

fn contains_pair(sequence: &Vec<u32>) -> bool {
    for i in 0..sequence.len() - 1 {
        if sequence[i] == sequence[i + 1] {
            return true;
        }
    }

    return false;
}

fn number_to_vec(number: u32) -> Vec<u32> {
    number.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect()
}