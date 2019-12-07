struct IntcodeComp {
    tape: Vec<i32>,
    head: usize,
    mode: i32,
}

impl IntcodeComp {
    pub fn new(t: Vec<i32>) -> IntcodeComp {
        IntcodeComp{
            tape: t,
            head: 0,
            mode: 0,
        }
    }

    pub fn execute(&mut self) {
        while self.tape[self.head] != 99 {
            self.execute_one();
        }
    }

    pub fn get(&self, i: usize) -> i32 {
        self.tape[i]
    }

    fn execute_one(&mut self) {
        let op = self.tape[self.head];
        self.head += 1;
        match op {
            1 => self.add(),
            2 => self.mult(),
            _ => panic!("Unsupported opcode!  Current computer state: opcode {}, head {}, mode {}",
                        op, self.head, self.mode),
        }
    }

    fn add(&mut self) {
        let x: usize = self.tape[self.head] as usize; self.head += 1;
        let y: usize = self.tape[self.head] as usize; self.head += 1;
        let out: usize = self.tape[self.head] as usize; self.head += 1;
        self.tape[out] = self.tape[x] + self.tape[y];
    }

    fn mult(&mut self) {
        let x: usize = self.tape[self.head] as usize; self.head += 1;
        let y: usize = self.tape[self.head] as usize; self.head += 1;
        let out: usize = self.tape[self.head] as usize; self.head += 1;
        self.tape[out] = self.tape[x] * self.tape[y];
    }
}

fn to_tape(line: &str) -> Vec<i32> {
    line.split(",").map(|p| p.parse::<i32>().unwrap()).collect()
}

fn main() {
    let line = std::fs::read_to_string("day-two/input.txt").expect("file not found");
    let tape: Vec<i32> = to_tape(&line);

    for noun in 0..100 {
        for verb in 0..100 {
            let mut tape_copy = tape.clone();
            tape_copy[1] = noun;
            tape_copy[2] = verb;
            let mut comp = IntcodeComp::new(tape_copy);
            comp.execute();
            let result = comp.get(0 as usize);
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
            let tape = to_tape(input);
            let mut comp = IntcodeComp::new(tape);
            comp.execute();
            assert_eq!(comp.get(0), *expected);
        }
    }
}