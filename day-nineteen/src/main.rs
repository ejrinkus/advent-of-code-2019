fn check(x: i64, y: i64, tape: Vec<i64>) -> bool {
    let mut comp = intcode::IntcodeComp::new(tape);
    comp.push_input(x);
    comp.push_input(y);
    comp.start();
    comp.pop_output().unwrap() != 0
}

fn part_one() {
    let line = std::fs::read_to_string("day-nineteen/input.txt").expect("file not found");
    let tape: Vec<i64> = intcode::to_tape(&line);

    let mut count = 0;
    for y in 0..50 {
        for x in 0..50 {
            if check(x, y, tape.clone()) { count += 1; }
        }
    }
    println!("Affected tiles: {}", count);
}

fn main() {
    part_one();
}
