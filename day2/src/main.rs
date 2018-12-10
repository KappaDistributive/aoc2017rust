
const INPUT: &str = include_str!("../input.txt");

fn solve_part_1(input_str: &str) -> u32 {
    let mut result: u32 = 0;
    for lines in input_str.trim().lines() {
        result += lines.split_whitespace().map(|w| w.parse::<u32>().unwrap()).max().unwrap();
        result -= lines.split_whitespace().map(|w| w.parse::<u32>().unwrap()).min().unwrap();
    }
    result
}

fn solve_part_2(input_str: &str) -> u32 {
    let mut result: u32 = 0;
    for lines in input_str.trim().lines() {
        let temp: Vec<u32> = lines.split_whitespace().map(|w| w.parse::<u32>().unwrap()).collect();
        result += subroutine(&temp);        
    }
    result
}

fn subroutine(temp: &Vec<u32>) -> u32 {
    for i in 0..temp.len() {
        for j in i+1..temp.len() {
            if temp[i] % temp[j] == 0 && (temp[j] / temp[i]) % 2 == 0 {
                return temp[i] / temp[j];
            }
            if temp[j] % temp[i] == 0 && (temp[i] / temp[j]) % 2 == 0 {
                return temp[j] / temp[i];
            }
        }
    }
    0
}

fn main() {
    println!("Answer part 1: {}",solve_part_1(INPUT));
    println!("Answer part 2: {}",solve_part_2(INPUT));
}
