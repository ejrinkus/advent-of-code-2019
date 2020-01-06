#[macro_use] extern crate text_io;

fn main() {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("input.txt");
    let line = std::fs::read_to_string(&path).expect("file not found");

    let tape: Vec<i64> = intcode::to_tape(&line);
    let mut comp = intcode::IntcodeComp::new(tape);
    comp.start();

    loop {
        while let Some(output) = comp.pop_output() {
            print!("{}", output as u8 as char);
        }

        let line: String = read!("{}\r\n");

        for c in line.as_bytes() {
            comp.push_input(*c as i64);
        }
        comp.push_input(10);

        comp.start();
    }
}
