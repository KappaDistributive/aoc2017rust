use std::collections::HashSet;

const INPUT: [u32;16] = [11, 11, 13, 7, 0, 15, 5, 5, 4, 4, 1, 1, 7, 1, 15, 11];

fn step(input: [u32;16]) -> [u32;16] {
    let mut config = input.clone();
    let mut max: u32 = 0;
    let mut index: usize = 0;
    for i in 0..16usize {
        if config[i] > max {
            max = config[i];
            index = i;
        }
    }
    let mut temp: u32 = config[index];
    config[index] = 0;
    for _ in 0..temp {
        index += 1;
        config[index % 16] += 1;
    }
    config
}

fn solve_part_1(input: [u32;16]) -> u32 {
    let mut config = input.clone();
    let mut steps: u32 = 0;
    let mut configs: HashSet<[u32;16]> = HashSet::new();
    loop {
        if configs.contains(&config) {
            break;
        }
        else {
            steps += 1;
            configs.insert(config);
            config = step(config);
        }
    }
    steps
}

fn solve_part_2(input: [u32;16]) -> u32 {
    let mut config = input.clone();
    let mut configs: HashSet<[u32;16]> = HashSet::new();
    loop {
        if configs.contains(&config) {
            break;
        }
        else {
            configs.insert(config);
            config = step(config);
        }
    }
    solve_part_1(config)
}


fn main() {
    println!("Answer part 1: {}", solve_part_1(INPUT));
    println!("Answer part 2: {}", solve_part_2(INPUT));
}
