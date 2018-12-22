
const INPUT: &str = include_str!("../input.txt");

fn score_garbabe(input_str: &str) -> (usize, usize) {
    let mut score: usize = 0;
    let mut garbage: usize = 0;
    let mut depth: usize = 0;
    let input = input_str.to_string();
    let mut chars  = input.chars();
    while let Some(c) = chars.next() {
        match c {
            '{' => {
                depth += 1;
            }
            '}' => {
                score += depth;
                depth -= 1;
            }
            '<' => {
                while let Some(d) = chars.next() {
                    match d {
                        '!' => {
                            chars.next();
                        }
                        '>' => {
                            break;
                        }
                        _ => {
                            garbage += 1;
                        }
                    }
                }
            }
            _ => {
                
            }
        }
    }
    (score, garbage)
}

fn solve_part_1(input_str: &str) -> usize {
    score_garbabe(input_str).0
}

fn solve_part_2(input_str: &str) -> usize {
    score_garbabe(input_str).1
}
fn main() {
    println!("Answer part 1: {}",solve_part_1(INPUT));
    println!("Answer part 2: {}",solve_part_2(INPUT));
}
