#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;
use std::collections::HashMap;
use std::cmp;

const INPUT: &str = include_str!("../input.txt");
const INPUT_SAMPLE: &str = include_str!("../input_sample.txt");

struct RegisterMachine {
    register: HashMap<String,i32>,
    instructions: Vec<Instruction>,
    pointer: usize,
}

impl RegisterMachine {
    fn new() -> Self {
        let register: HashMap<String,i32> = HashMap::new();
        let instructions: Vec<Instruction> = Vec::new();
        let pointer: usize = 0;
        RegisterMachine{ register, instructions, pointer }
    }

    fn from_str(instructions_str: &str) -> Self {
        let register: HashMap<String,i32> = HashMap::new();
        let mut instructions: Vec<Instruction> = Vec::new();
        let pointer: usize = 0;
        
        for line in instructions_str.trim().lines() {
            instructions.push(Instruction::from_str(line));
        }

        RegisterMachine { register, instructions, pointer }
    }

    fn step(&mut self) {
        let (acc,op,acc_change,reg,cmp,val) = self.instructions[self.pointer].data();
        match self.register.get(&acc) {
            Some(v) => {
                
            }
            None => {
                self.register.insert(acc.clone(),0);
            }
        }
        match self.register.get(&reg) {
            Some(v) => {

            }
            None => {
                self.register.insert(reg.clone(),0);
            }
        }
        let acc_value: i32 = *self.register.get(&acc).unwrap();
        let mut execute: bool = false;
        match cmp.as_str() {
            "<=" => {
                execute = *self.register.get(&reg).unwrap() <= val;
            }
            ">=" => {
                execute = *self.register.get(&reg).unwrap() >= val;
            }
            "<" => {
                execute = *self.register.get(&reg).unwrap() < val;
            }
            ">" => {
                execute = *self.register.get(&reg).unwrap() > val;
            }
            "==" => {
                execute = *self.register.get(&reg).unwrap() == val;
            }
            "!=" => {
                execute = *self.register.get(&reg).unwrap() != val;
            }
            _ => {
                panic!("Unknown comparator: {}", cmp);
            }
        }
        if execute {
            match op.as_str() {
                "dec" => {
                    self.register.insert(acc, acc_value - acc_change);
                }
                "inc" => {
                    self.register.insert(acc, acc_value + acc_change);
                }
                _ => {
                    panic!("Unknown operation: {}", op);
                }
            }
        }
        self.pointer += 1;
    }

    fn print_register(&self) {
        for (reg,val) in self.register.iter() {
            println!("{} = {}", reg, val);
        }
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
            static ref RE_INS: Regex =
                Regex::new(r"(?P<acc>[a-z]+) (?P<op>[a-z]+) (?P<acc_change>-?[0-9]+) if (?P<reg>[a-z]+) (?P<cmp>[!><=]+) (?P<val>-?[0-9]+)").unwrap();
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

    fn data(&self) -> (String, String, i32, String, String, i32) {
        (self.acc.clone(), self.op.clone(), self.acc_change, self.reg.clone(), self.cmp.clone(), self.val)
    }
}

fn solve_part_1(input_str: &str) -> i32 {
    let mut rm: RegisterMachine = RegisterMachine::from_str(input_str);
    for _ in 0..rm.instructions.len() {
        rm.step();
    }
    let mut result: i32 = std::i32::MIN;
    for (_,v) in rm.register.iter() {
        result = cmp::max(result, *v);
    }
    result
}

fn main() {
    println!("Answer part 1: {}", solve_part_1(INPUT));
    
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
