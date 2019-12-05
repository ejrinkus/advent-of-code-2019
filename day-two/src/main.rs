use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2, "Incorrect number of args");

    let f = File::open(&args[1]).expect("file not found");
    let reader = BufReader::new(&f);
    for line in reader.lines() {
        let mut tape: Vec<i32> = Vec::new();
        let string = line.unwrap();
        let pieces: Vec<&str> = string.split(",").collect();
        for piece in pieces {
            let val = piece.parse::<i32>().unwrap();
            tape.push(val);
        }
        run_tape(tape);
    }
}

fn run_tape(mut tape: Vec<i32>) {
    let mut head: usize = 0 as usize;
    while tape[head] != 99 {
        let op = tape[head];
        let x = tape[tape[head+1] as usize];
        let y = tape[tape[head+2] as usize];
        let out: usize = tape[head+3] as usize;

        if op == 1 {
            tape[out] = x + y;
        } else if op == 2 {
            tape[out] = x * y;
        }

        head += 4;
    }

    for entry in tape {
        print!("{},", entry);
    }
}