extern crate regex;
#[macro_use]
extern crate lazy_static;

use regex::Regex;
use std::string::String;

const INPUT: &str = include_str!("../input.txt");

#[derive(Clone)]
struct Program {
    name: String,
    weight: u32,
    disk: Vec<String>,
}


impl PartialEq for Program {
    fn eq(&self, other: &Program) -> bool {
        self.name == other.name
    }
}

fn program_is_listed(prgn: &str, programs: &Vec<Program>) -> bool {
    for p in programs {
        if p.name == *prgn {
            return true;
        }
    }
    false
}

fn format_input(input_str: &str) -> Vec<Program> {
    lazy_static! {
        static ref RE_LEAF: Regex = Regex::new(r"(?P<name>[a-z]+)\s\((?P<weight>[0-9]+)\)$").unwrap();
        static ref RE_INNER: Regex = Regex::new(r"(?P<name>[a-z]+)\s\((?P<weight>[0-9]+)\) -> (?P<disk>[a-z,\s]+)$").unwrap();
    }
    let mut programs: Vec<Program> = Vec::new();
    // create leaves
    for line in input_str.trim().lines() {
        match RE_LEAF.captures(line) {
            Some(caps) => {
                let name = String::from(caps.name("name").map_or("", |p| p.as_str()));
                let weight_str = caps.name("weight").map_or("", |p| p.as_str());
                let weight: u32 = weight_str.parse::<u32>().unwrap();
                let disk: Vec<String> = Vec::new();
                let program: Program = Program { name, weight, disk };
                if !programs.contains(&program) {
                    programs.push(program);
                }
            }
            None => {
                continue;
            }
        }
    }

    // create inner programs
    for line in input_str.trim().lines() {
        match RE_INNER.captures(line) {
            Some(caps) => {
                // check if all programs on the disk have already been
                // created
                let disk_str = caps.name("disk").map_or("", |p| p.as_str());
                let name = String::from(caps.name("name").map_or("", |p| p.as_str()));
                let weight_str = caps.name("weight").map_or("", |p| p.as_str());
                let weight: u32 = weight_str.parse::<u32>().unwrap();
                let mut disk: Vec<String> = Vec::new();
                for progn in disk_str.trim().split(", ") {
                    disk.push(String::from(progn));
                }
                let program: Program = Program { name, weight, disk };
                if !programs.contains(&program) {
                    programs.push(program);
                }
            }
            
            None => {
                continue;
            }
        }
    }
    programs
}

fn predecessor(program: &Program, programs: &Vec<Program>) -> Option<Program> {
    for p in programs {
        for q in p.disk.iter() {
            if program.name == *q {
                return Some(p.clone());
            }
        }
    }
    None
}

fn solve_part_1(input_str: &str) -> String {
    let mut programs: Vec<Program> = format_input(input_str);
    let mut program = programs[0].clone();
    loop {
        match predecessor(&program, &programs) {
            Some(p) => {
                program = p;
            }
            None => {
                return program.name;
            }
        }
    }
}

    

fn main() {
    println!("{}", solve_part_1(INPUT));
}
