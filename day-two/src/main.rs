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
        let mut noun = 0;
        let mut verb = 0;
        while noun <= 99 {
            while verb <= 99 {
                let result = run_tape(tape.clone(), noun, verb);
                if result == 19690720 {
                    println!("{}{}", noun, verb);
                    noun = 99;
                    verb = 99;
                }
                verb += 1;
            }
            noun += 1;
            verb = 0;
        }
    }
}

fn run_tape(mut tape: Vec<i32>, noun: i32, verb: i32) -> i32 {
    tape[1] = noun;
    tape[2] = verb;
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

    return tape[0];
}