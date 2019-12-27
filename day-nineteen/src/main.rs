fn check(x: i64, y: i64, tape: Vec<i64>) -> bool {
    let mut comp = intcode::IntcodeComp::new(tape);
    comp.push_input(x);
    comp.push_input(y);
    comp.start();
    comp.pop_output().unwrap() != 0
}

fn part_one(tape: &Vec<i64>) {
    let mut count = 0;
    for y in 0..50 {
        for x in 0..50 {
            if check(x, y, tape.clone()) { count += 1; }
        }
    }
    println!("Affected tiles: {}", count);
}

fn part_two(tape: &Vec<i64>) {
    let mut bl = (0, 99);
    let mut ur = (99, 0);
    loop {
        // Move our window right until the bottom left
        // corner is in the beam.
        while !check(bl.0, bl.1, tape.clone()) {
            bl.0 += 1;
            ur.0 += 1;
        }
        // Move our window down until the upper right
        // corner is in the beam.
        while !check(ur.0, ur.1, tape.clone()) {
            bl.1 += 1;
            ur.1 += 1;
        }
        // If the bottom left is still in the beam, then
        // we've found our window.
        if check(bl.0, bl.1, tape.clone()) {
            break;
        }
    }
    println!("Found window at ({}, {})", bl.0, ur.1);
}

fn main() {
    let line = std::fs::read_to_string("day-nineteen/input.txt").expect("file not found");
    let tape: Vec<i64> = intcode::to_tape(&line);
    part_one(&tape);
    part_two(&tape);
}
