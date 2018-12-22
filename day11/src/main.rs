use std::ops::AddAssign;
use std::fmt::{Display, Formatter, Result};
use self::Step::*;

const INPUT: &str = include_str!("../input.txt");

#[derive(Debug)]
enum Step {
    N,
    NW,
    SW,
    S,
    SE,
    NE,
}

impl Step {
    fn parse_step(input: &str) -> Option<Step> {
        let result = match input {
            "n" => N,
            "nw" => NW,
            "sw" => SW,
            "s" => S,
            "se" => SE,
            "ne" => NE,
            _ => return None,
        };
        Some(result)
    }

    fn step(&self) -> Point {
        match *self {
            N => Point{x:0,y:-1,z:1},
            NW => Point{x:-1,y:0,z:1},
            SW => Point{x:-1,y:1,z:0},
            S => Point{x:0,y:1,z:-1},
            SE => Point{x:1,y:0,z:-1},
            NE => Point{x:1,y:-1,z:0},
        }
    }
}




struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl AddAssign for Point {
    fn add_assign(&mut self, other: Point) {
        *self = Point { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z };
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

fn format_input(input: &str) -> Vec<Step> {
    input.trim().split(',').map(Step::parse_step).flat_map(|v| v).collect()

}

fn solve_part_1(input_str: &str) -> u32 {
    let steps: Vec<Step> = format_input(input_str);
    let mut pos: Point = Point{ x: 0, y: 0, z: 0 };
    for step in steps {
        pos += step.step();
    }
    ((pos.x.abs() + pos.y.abs() + pos.z.abs())/2) as u32
}


// information about hexagonal grids and their distance functions
// taken from https://www.redblobgames.com/grids/hexagons/
fn main() {
    println!("Answer part 1: {}",solve_part_1(INPUT));
}
