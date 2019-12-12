fn main() {
    let line = std::fs::read_to_string("day-nine/input.txt").expect("file not found");
    let tape: Vec<i64> = intcode::to_tape(&line);
    
    let mut comp = intcode::IntcodeComp::new(tape.clone());
    comp.start();

    loop {
        match comp.pop_output() {
            Some(o) => println!("{}", o),
            None => break,
        }
    }
}