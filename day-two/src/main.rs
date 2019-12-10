fn main() {
    let line = std::fs::read_to_string("day-two/input.txt").expect("file not found");
    let tape: Vec<i32> = intcode::to_tape(&line);

    for noun in 0..100 {
        for verb in 0..100 {
            let mut tape_copy = tape.clone();
            tape_copy[1] = noun;
            tape_copy[2] = verb;
            let mut comp = intcode::IntcodeComp::new(tape_copy);
            comp.start();
            let result = comp.get(0 as usize);
            if result == 19690720 {
                println!("{}{}", noun, verb);
                return;
            }
        }
    }
}