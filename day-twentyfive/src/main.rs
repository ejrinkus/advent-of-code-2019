#[macro_use] extern crate text_io;

fn auto_win(tape: Vec<i64>) {
    let mut comp = intcode::IntcodeComp::new(tape);
    let quickstart = "east
east
take semiconductor
north
north
take antenna
south
west
take food ration
west
west
take monolith
east
east
east
south
east
south
south
east
east
";

    comp.start();

    for line in quickstart.lines() {
        while let Some(_) = comp.pop_output() {
            continue;
        }

        for c in line.as_bytes() {
            comp.push_input(*c as i64);
        }
        comp.push_input(10);
        comp.start();
    }

    while let Some(output) = comp.pop_output() {
        print!("{}", output as u8 as char);
    }
}

fn just_play(tape: Vec<i64>) {
    let mut comp = intcode::IntcodeComp::new(tape);
    comp.start();
    while *comp.state() != intcode::IntcodeState::Finished {
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
    while let Some(output) = comp.pop_output() {
        print!("{}", output as u8 as char);
    }
}

fn main() {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("input.txt");
    let line = std::fs::read_to_string(&path).expect("file not found");

    let tape: Vec<i64> = intcode::to_tape(&line);
    auto_win(tape.clone());
    just_play(tape.clone());
}
