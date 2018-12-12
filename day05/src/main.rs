
const INPUT: &str = include_str!("../input.txt");

fn format_input(input_str: &str) -> Vec<i32> {
    let mut jumps: Vec<i32> = Vec::new();
    for line in input_str.trim().lines() {
        jumps.push(line.parse::<i32>().unwrap());
    }
    jumps
}

fn solve_part_1(input_str: &str) -> u32 {
    let mut jumps: Vec<i32> = format_input(input_str);
    let mut i: i32 = 0;
    let mut steps: u32 = 0;
    while 0 <= i && i < jumps.len() as i32 {
        jumps[i as usize] += 1;
        steps += 1;
        i += jumps[i as usize] - 1;
    }
    steps
}

fn solve_part_2(input_str: &str) -> u32 {
    let mut jumps: Vec<i32> = format_input(input_str);
    let mut i: i32 = 0;
    let mut steps: u32 = 0;
    while 0 <= i && i < jumps.len() as i32 {
        steps += 1;
        if jumps[i as usize] >= 3 {
            jumps[i as usize] -= 1;
            i += jumps[i as usize] + 1;
        }
        else {
            jumps[i as usize] += 1;
            i += jumps[i as usize] - 1;
        }
    }
    steps
}

fn main() {
    println!("Answer part 1: {}", solve_part_1(INPUT));
    println!("Answer part 2: {}", solve_part_2(INPUT));
}
