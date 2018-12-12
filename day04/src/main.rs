use std::collections::HashSet;

const INPUT: &str = include_str!("../input.txt");

fn solve_part_1(input_str: &str) -> u32 {
    let mut result: u32 = 0;
    for line in input_str.trim().lines() {
        let mut temp: HashSet<String> = HashSet::new();
        let mut legal:bool = true;
        for word in line.trim().split_whitespace() {
            if temp.contains(&String::from(word)) {
                legal = false;
                break;
            }
            temp.insert(String::from(word));
        }
        if legal {
            result += 1;
        }
    }
    result
}

fn solve_part_2(input_str: &str) -> u32 {
    let mut result: u32 = 0;
    for line in input_str.trim().lines() {
        let mut temp: HashSet<Vec<char>> = HashSet::new();
        let mut legal:bool = true;
        for word in line.trim().split_whitespace() {
            let mut reduct: Vec<char> = word.chars().collect();
            reduct.sort_by(|a,b| b.cmp(a));
            if temp.contains(&reduct) {
                legal = false;
                break;
            }            
            temp.insert(reduct);
        }
        if legal {
            result += 1;
        }
    }
    result
}

fn main() {
    println!("Answer part 1: {}", solve_part_1(INPUT));
    println!("Answer part 2: {}", solve_part_2(INPUT));
}
