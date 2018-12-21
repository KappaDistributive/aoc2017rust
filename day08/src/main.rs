#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;
use std::collections::HashMap;

const INPUT: &str = include_str!("../input.txt");
const INPUT_SAMPLE: &str = include_str!("../input_sample.txt");

struct RegisterMachine {
    register: HashMap<String,i32>,
    instructions: Vec<Instruction>
}

impl RegisterMachine {
    fn new() -> Self {
        let register: HashMap<String,i32> = HashMap::new();
        let instructions: Vec<Instruction> = Vec::new();
        RegisterMachine{ register, instructions }
    }

    fn from_str(instructions_str: &str) -> Self {
        let register: HashMap<String,i32> = HashMap::new();
        let mut instructions: Vec<Instruction> = Vec::new();

        for line in instructions_str.trim().lines() {
            instructions.push(Instruction::from_str(line));
        }

        RegisterMachine { register, instructions }
    }
}

struct Instruction {
    acc: String,
    op: String,
    acc_change: i32,
    reg: String,
    cmp: String,
    val: i32,
}

impl Instruction {
    fn from_str(instructions_str: &str) -> Self {
        lazy_static!{
            static ref RE_INS: Regex = Regex::new(r"(?P<acc>[a-z]+) (?P<op>[a-z]+) (?P<acc_change>-?[0-9]+) if (?P<reg>[a-z]+) (?P<cmp>[!><=]+) (?P<val>-?[0-9]+)").unwrap();
        }
        match RE_INS.captures(instructions_str) {
            Some(cap) => {
                return Instruction { acc: cap.name("acc").map_or("".to_string(), |m| m.as_str().to_string()),
                                     op: cap.name("op").map_or("".to_string(), |m| m.as_str().to_string()),
                                     acc_change: cap.name("acc_change").map_or(0, |m| m.as_str().parse::<i32>().unwrap()),
                                     reg: cap.name("reg").map_or("".to_string(), |m| m.as_str().to_string()),
                                     cmp: cap.name("cmp").map_or("".to_string(), |m| m.as_str().to_string()),
                                     val: cap.name("val").map_or(0, |m| m.as_str().parse::<i32>().unwrap()) };
            }
            None => {
                panic!("Couldn't parse {} as instruction", instructions_str);
            }
        }
        
    }
}

fn main() {

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_instruction_from_str() {
        let challenge: Instruction = Instruction { acc: String::from("b"),
                                                   op: String::from("inc"),
                                                   acc_change: 5,
                                                   reg: String::from("a"),
                                                   cmp: String::from(">"),
                                                   val: 1};
        let answer: Instruction = Instruction::from_str(&"b inc 5 if a > 1");
        assert_eq!(challenge.acc, answer.acc);
        assert_eq!(challenge.op, answer.op);
        assert_eq!(challenge.acc_change, answer.acc_change);
        assert_eq!(challenge.reg, answer.reg);
        assert_eq!(challenge.cmp, answer.cmp);
        assert_eq!(challenge.val, answer.val);
    }

}
