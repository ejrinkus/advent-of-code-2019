#[macro_use] extern crate text_io;

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
        let op = self.tape[self.head] % 100;
        self.mode = self.tape[self.head] / 100;
        self.head += 1;
        match op {
            1 => self.add(),
            2 => self.mult(),
            3 => self.input(),
            4 => self.output(),
            5 => self.jump_if_true(),
            6 => self.jump_if_false(),
            7 => self.less_than(),
            8 => self.equals(),
            _ => panic!("Unsupported opcode!  Current computer state: opcode {}, head {}, mode {}",
                        op, self.head, self.mode),
        }
    }

    fn get_input_param(&mut self) -> i32 {
        let m = self.mode % 10;
        self.mode /= 10;
        match m {
            0 => {
                let pos = self.tape[self.head] as usize;
                self.head += 1;
                return self.tape[pos];
            },
            1 => {
                let val = self.tape[self.head];
                self.head += 1;
                return val;
            }
            _ => panic!("Unsupported mode!  Current computer state: m {}, head {}, mode {}",
                        m, self.head, self.mode),
        }
    }

    fn add(&mut self) {
        let x = self.get_input_param();
        let y = self.get_input_param();
        let pos: usize = self.tape[self.head] as usize; self.head += 1;
        self.tape[pos] = x + y;
    }

    fn mult(&mut self) {
        let x = self.get_input_param();
        let y = self.get_input_param();
        let pos: usize = self.tape[self.head] as usize; self.head += 1;
        self.tape[pos] = x * y;
    }

    fn input(&mut self) {
        println!("Input requested: ");
        let input: i32 = read!();
        let pos: usize = self.tape[self.head] as usize; self.head += 1;
        self.tape[pos] = input;
    }

    fn output(&mut self) {
        let out = self.get_input_param();
        println!("OUTPUT: {}", out);
    }

    fn jump_if_true(&mut self) {
        let x = self.get_input_param();
        let y = self.get_input_param();
        if x != 0 {
            self.head = y as usize;
        }
    }

    fn jump_if_false(&mut self) {
        let x = self.get_input_param();
        let y = self.get_input_param();
        if x == 0 {
            self.head = y as usize;
        }
    }

    fn less_than(&mut self) {
        let x = self.get_input_param();
        let y = self.get_input_param();
        let pos: usize = self.tape[self.head] as usize; self.head += 1;
        if x < y {
            self.tape[pos] = 1;
        } else {
            self.tape[pos] = 0;
        }
    }

    fn equals(&mut self) {
        let x = self.get_input_param();
        let y = self.get_input_param();
        let pos: usize = self.tape[self.head] as usize; self.head += 1;
        if x == y {
            self.tape[pos] = 1;
        } else {
            self.tape[pos] = 0;
        }
    }
}

fn to_tape(line: &str) -> Vec<i32> {
    line.split(",").map(|p| p.parse::<i32>().unwrap()).collect()
}

fn main() {
    let line = std::fs::read_to_string("day-five/input.txt").expect("file not found");
    let mut tape: Vec<i32> = to_tape(&line);

    let mut comp = IntcodeComp::new(tape);
    comp.execute();
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