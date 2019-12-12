struct ComboGenerator {
    pub combo: Vec<i64>,
    c: Vec<i64>,
    n: u8,
    i: usize,
}

impl ComboGenerator {
    fn new(start: Vec<i64>) -> ComboGenerator {
        let length = start.len();
        ComboGenerator {
            combo: start,
            c: vec![0; length as usize],
            n: length as u8,
            i: 0,
        }
    }

    fn next(&mut self) -> bool {
        while self.i < self.n as usize {
            if self.c[self.i] < self.i as i64 {
                if self.i % 2 == 0 {
                    let temp = self.combo[0];
                    self.combo[0] = self.combo[self.i];
                    self.combo[self.i] = temp;
                } else {
                    let temp = self.combo[self.c[self.i] as usize];
                    self.combo[self.c[self.i] as usize] = self.combo[self.i];
                    self.combo[self.i] = temp;
                }
                //Swap has occurred ending the for-loop. Simulate the increment of the for-loop counter
                self.c[self.i] += 1;
                //Simulate recursive call reaching the base case by bringing the pointer to the base case analog in the array
                self.i = 0;
                return true;
            } else {
                self.c[self.i] = 0;
                self.i += 1;
            }
        }
        return false;
    }
}

fn try_no_feedback(tape: Vec<i64>, phases: Vec<i64>) -> i64 {
    let mut signal = 0;
    for phase in phases {
        let mut comp = intcode::IntcodeComp::new(tape.clone());
        comp.push_input(phase);
        comp.push_input(signal);
        comp.start();
        match comp.state() {
            intcode::IntcodeState::Finished => (),
            _ => panic!("Unexpected state: {:?}", comp.state()),
        }
        match comp.pop_output() {
            Some(o) => signal = o,
            None => panic!("Failed to get output"),
        }
    }
    signal
}

fn try_feedback(tape: Vec<i64>, phases: Vec<i64>) -> i64 {
    let mut comp_a = intcode::IntcodeComp::new(tape.clone());
    comp_a.push_input(phases[0]);
    let mut comp_b = intcode::IntcodeComp::new(tape.clone());
    comp_b.push_input(phases[1]);
    let mut comp_c = intcode::IntcodeComp::new(tape.clone());
    comp_c.push_input(phases[2]);
    let mut comp_d = intcode::IntcodeComp::new(tape.clone());
    comp_d.push_input(phases[3]);
    let mut comp_e = intcode::IntcodeComp::new(tape.clone());
    comp_e.push_input(phases[4]);

    let mut comps = vec![comp_a, comp_b, comp_c, comp_d, comp_e];
    let mut signal = 0;
    let mut i = 0;
    loop {
        comps[i].push_input(signal);
        comps[i].start();
        match comps[i].pop_output() {
            Some(o) => signal = o,
            None => println!("Unexpected?"),
        }
        i += 1;
        if i >= comps.len() {
            if *comps[i-1].state() == intcode::IntcodeState::Finished {
                break;
            }
            i = 0;
        }
    }
    signal
}

fn main() {
    let line = std::fs::read_to_string("day-seven/input.txt").expect("file not found");
    let tape: Vec<i64> = intcode::to_tape(&line);
    
    let mut generator = ComboGenerator::new((0..5).collect());
    let mut largest_signal = 0;
    loop {
        let signal = try_no_feedback(tape.clone(), generator.combo.clone());
        if signal > largest_signal {
            largest_signal = signal;
        }
        if !generator.next() { break; }
    }
    println!("Max possible signal to thrusters (no feedback): {}", largest_signal);
    
    let mut generator = ComboGenerator::new((5..10).collect());
    let mut largest_signal = 0;
    loop {
        let signal = try_feedback(tape.clone(), generator.combo.clone());
        if signal > largest_signal {
            largest_signal = signal;
        }
        if !generator.next() { break; }
    }
    println!("Max possible signal to thrusters (no feedback): {}", largest_signal);
}