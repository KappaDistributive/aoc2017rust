
fn solve_part_1(lenghts: Vec<usize>) -> usize {
    let mut circle: [usize;256] = [0;256];
    for i in 0..256 {
        circle[i] = i;
    }
    let mut position: usize = 0;
    let mut skip_size: usize = 0;
    for length in lenghts {
        for i in 0..(length / 2) {
            let temp = circle[(position + i) % 256];
            circle[(position + i) % 256] = circle[(position + length - 1 - i) % 256];
            circle[(position + length - 1 - i) % 256] = temp;
        }
        position += length + skip_size;
        skip_size += 1;
    }
    circle[0] * circle[1]
}

fn main() {
    let lengths: Vec<usize> = vec![31,2,85,1,80,109,35,63,98,255,0,13,105,254,128,33];

    println!("{}", solve_part_1(lengths));
}
