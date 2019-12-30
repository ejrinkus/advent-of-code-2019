fn main() {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("input.txt");
    let input = std::fs::read_to_string(&path).expect("file not found");
    let tape: Vec<i64> = intcode::to_tape(&input);

    let mut comp = intcode::IntcodeComp::new(tape);
    comp.start();

    while let Some(output) = comp.pop_output() {
        print!("{}", output as u8 as char);
    }

    let instructions = "NOT A J
NOT B T
OR T J
NOT C T
OR T J
AND D J
NOT E T
NOT T T
OR H T
AND T J
RUN\n";

    print!("{}", instructions);
    for c in instructions.as_bytes() {
        comp.push_input(*c as i64);
    }
    comp.start();

    while let Some(output) = comp.pop_output() {
        if output < 256 {
            print!("{}", output as u8 as char);
        } else {
            print!("{}", output);
        }
    }
}
