#[macro_use] extern crate text_io;

fn main() {
    let line = std::fs::read_to_string("day-nine/input.txt").expect("file not found");
    let tape: Vec<i64> = intcode::to_tape(&line);
    
    let mut comp = intcode::IntcodeComp::new(tape.clone());
    comp.start();

    while *comp.state() != intcode::IntcodeState::Finished {
        match comp.state() {
            intcode::IntcodeState::NeedsInput => {
                println!("Input requested: ");
                let input: i64 = read!();
                comp.push_input(input);
                comp.start();
            },
            intcode::IntcodeState::Err(s) => panic!("Unexpected error: {}", s),
            _ => (),
        }
    }

    loop {
        match comp.pop_output() {
            Some(o) => println!("{}", o),
            None => break,
        }
    }
}