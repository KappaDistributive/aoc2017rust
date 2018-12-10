
const INPUT: &str = include_str!("../input.txt");

fn solve_part_1(input_str: &str) -> u32 {
    const RADIX: u32 = 10;
    let mut result: u32 = 0;
    let input: Vec<u32> = input_str.trim().chars().map(|c| c.to_digit(RADIX).unwrap()).collect();
    for i in 0..input.len() {
        if input[i] == input[(i+1) % input.len()] {
            result += input[i];
        }
    }
    result
}

fn solve_part_2(input_str: &str) -> u32 {
    const RADIX: u32 = 10;
    let mut result: u32 = 0;
    let input: Vec<u32> = input_str.trim().chars().map(|c| c.to_digit(RADIX).unwrap()).collect();
    let len: usize = input.len();
    for i in 0..len {
        if input[i] == input[(i+ len/2) % input.len()] {
            result += input[i];
        }
    }
    result
}

fn main() {
    println!("Answer part 1: {}", solve_part_1(INPUT));
    println!("Answer part 2: {}", solve_part_2(INPUT));
}
