fn to_tape(line: &str) -> Vec<i32> {
    line.split(",").map(|p| p.parse::<i32>().unwrap()).collect()
}

fn add(tape: &mut Vec<i32>, x: usize, y: usize, out: usize) {
    tape[out] = tape[x] + tape[y];
}

fn mult(tape: &mut Vec<i32>, x: usize, y: usize, out: usize) {
    tape[out] = tape[x] * tape[y];
}

fn run_tape(mut tape: Vec<i32>) -> i32 {
    let mut head: usize = 0 as usize;
    while tape[head] != 99 {
        match tape[head] {
            1 => {
                let x = tape[head+1] as usize;
                let y = tape[head+2] as usize;
                let out = tape[head+3] as usize;
                add(&mut tape, x, y, out);
                head += 4;
            },
            2 => {
                let x = tape[head+1] as usize;
                let y = tape[head+2] as usize;
                let out = tape[head+3] as usize;
                mult(&mut tape, x, y, out);
                head += 4;
            },
            99 => break,
            _ => panic!("Unsupported op code {} at index {}", tape[head], head),
        }
    }

    tape[0]
}

fn main() {
    let line = std::fs::read_to_string("day-two/input.txt").expect("file not found");
    let tape: Vec<i32> = to_tape(&line);

    for noun in 0..100 {
        for verb in 0..100 {
            let mut tape_copy = tape.clone();
            tape_copy[1] = noun;
            tape_copy[2] = verb;
            let result = run_tape(tape_copy);
            if result == 19690720 {
                println!("{}{}", noun, verb);
                return;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_tape() {
        let cases = vec![
            ("1,0,0,0,99", 2),
            ("2,3,0,3,99", 2),
            ("2,4,4,5,99,0", 2),
            ("1,1,1,4,99,5,6,0,99", 30),
        ];

        for (input, expected) in cases.iter() {
            assert_eq!(run_tape(to_tape(input)), *expected);
        }
    }
}