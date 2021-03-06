#![crate_name = "intcode"]

use std::collections::VecDeque;

#[derive(Debug, PartialEq)]
pub enum IntcodeState {
    /// Indicates the computer is initialized and ready to start.
    Ready,
    /// Indicates the computer is currently running.
    Running,
    /// Indicates the computer has paused.  To resume, first provide input via
    /// the push_input() function.  Then call continue() to resume execution
    /// from where the computer left off.
    NeedsInput,
    /// Indicates the computer has successfully finished.
    Finished,
    /// Indicates the computer encountered an unexpected error.  An error
    /// message will be included if this result value is returned.
    Err(&'static str),
}

/// Contains all the state necessary for an Intcode Computer (an FSA written
/// for AoC 2019).
/// 
/// Every tape instruction consist of an opcode and input mode.  Each
/// instruction is followed by a number of parameters (the number of parameters
/// is different based on the opcode). The opcode is stored in the rightmost
/// two digits of the instruction.  These are the valid opcodes:
///  * 01 - Add.  Adds the values in the first and second parameters, and stores
///         the result in the position given by the third parameter.
///  * 02 - Multiply.  Multiplies the values in the first and second parameters,
///         and stores the result in the position given by the third parameter.
///  * 03 - Input.  Takes an input value from the computer's input queue (or
///         requests one from the client) and stores it in the position given
///         by the only parameter.
///  * 04 - Output.  Pushes the value of its only parameter into the computer's
///         output queue.
///  * 05 - Jump-if-true.  If the first parameter is non-zero, moves the
///         computer's head to the index provided in the second parameter.
///  * 06 - Jump-if-false.  If the first parameter is zero, moves the
///         computer's head to the index provided in the second parameter.
///  * 07 - Less than.  If the first parameter is less than the second
///         parameter, stores 1 in the position given by the third parameter.
///         Otherwise, stores 0 in the position given by the third parameter.
///  * 08 - Equals.  If the first parameter is equal to the second parameter
///         parameter, stores 1 in the position given by the third parameter.
///         Otherwise, stores 0 in the position given by the third parameter.
///  * 09 - Relative adjust.  Adjusts the relative base by the value in the
///         only parameter.  Value can be positive or negative, but the
///         relative base cannot be negative.  Relative base starts at 0 when
///         the computer is initialized.
/// 
/// The input mode consists of the digits preceding the opcode (one digit per
/// parameter on the instruction).  The mode digits are read right-to-left.
/// These are the supported modes:
///  * 0 - Positional.  The parameter contains a position.  The value for the
///        parameter should be read from that position on the tape.
///  * 1 - Immediate.  The value of the parameter itself should be used.
///  * 2 - Relative.  Similar to position, but rather than representing an
///        absolute position, the parameter represents an offset from the
///        relative base.
/// 
/// Note: Leading 0's are trimmed from instructions.  So an instruction of '1'
/// is an 'add' instruction where all parameters are in the positional mode
/// (this is equivalent to '00001').  Similarly, 1102 is equivalent to 01102.
/// Output parameters will always be in 'positional' mode.
pub struct IntcodeComp {
    /// The 'tape' that contains the instructions for the computer.
    tape: Vec<i64>,
    /// Points to the value on the tape that is about to be read and processed.
    head: usize,
    /// The current parameter mode of the computer.
    mode: i64,
    /// The starting point for any relative-mode parameters.
    rel_base: i64,
    /// Indicates the current result
    state: IntcodeState,
    /// A queue of inputs that have been provided to the computer.
    inputs: VecDeque<i64>,
    /// A queue of outputs generated by the computer.
    outputs: VecDeque<i64>,
}

impl IntcodeComp {
    /// Constructs and returns a new Intcode Computer, using a given tape as input.
    /// 
    /// # Examples
    /// 
    /// ```
    /// let tape: Vec<i64> = vec![99];
    /// let mut comp = intcode::IntcodeComp::new(tape);
    /// comp.start();
    /// assert_eq!(*comp.state(), intcode::IntcodeState::Finished);
    /// ```
    pub fn new(t: Vec<i64>) -> IntcodeComp {
        IntcodeComp{
            tape: t,
            head: 0,
            mode: 0,
            rel_base: 0,
            state: IntcodeState::Ready,
            inputs: VecDeque::new(),
            outputs: VecDeque::new(),
        }
    }

    /// Starts the intcode computer.  This function blocks, and will only
    /// return under 3 circumstances:
    ///  * The computer is finished executing.
    ///  * The computer needs more input.  The user must add more input using
    ///    the push_input() function first, then call start() again to resume
    ///    execution.
    ///  * The computer encountered a fatal error.
    /// 
    /// The state of the computer is an IntcodeState, and will be set to
    /// one of three values based on the above situations (Finished,
    /// NeedsInput, and Err respectively).
    pub fn start(&mut self) {
        match self.state {
            // If the computer is finished (or encountered an error), then
            // there's nothing to run.  So return right away.
            IntcodeState::Finished => return,
            IntcodeState::Err(_s) => return,
            _ => (),
        }
        let mut is_resume = self.state == IntcodeState::NeedsInput;
        self.state = IntcodeState::Running;
        while self.get(self.head) != 99 {
            if is_resume {
                // The computer previously paused because it needed more input.
                // Pick up where it left off by calling input() directly.
                self.input();
                is_resume = false;
                continue;
            }
            self.execute_one();
            if self.state != IntcodeState::Running {
                return;
            }
        }
        self.state = IntcodeState::Finished;
    }

    /// Given an index, return the value in the corresponding cell on the
    /// computer's tape.  If the given position is beyond the tape's bounds,
    /// this function will allocate additional memory at the end of the tape
    /// so make the tape large enough to include the given position.
    /// 
    /// Accessing a position that is beyond the tape's bounds will simply
    /// return a default value of zero.
    /// 
    /// # Example
    /// 
    /// ```
    /// let tape: Vec<i64> = vec![1101,2,3,0,1102,2,3,4,99];
    /// let mut comp = intcode::IntcodeComp::new(tape);
    /// comp.start();
    /// assert_eq!(*comp.state(), intcode::IntcodeState::Finished);
    /// assert_eq!(comp.get(0), 5);
    /// assert_eq!(comp.get(4), 6);
    /// ```
    pub fn get(&mut self, i: usize) -> i64 {
        if i >= self.tape.len() {
            return 0;
        }
        self.tape[i]
    }

    /// Set the value at a given index on the tape.  If the given position is
    /// beyond the tape's bounds, this function will allocate additional memory
    /// at the end of the tape so make the tape large enough to include the
    /// given position.
    fn set(&mut self, i: usize, v: i64) {
        if i >= self.tape.len() {
            self.tape.resize(i+1, 0);
        }
        self.tape[i] = v;
    }

    /// Push input into the computer's input queue.  This function can be used
    /// proactively when the user knows that the computer will need input.  Or
    /// it can be used reactively, when the computer pauses in the NeedsInput
    /// state, before resuming.
    /// 
    /// # Example
    /// 
    /// ```
    /// // This tape will take two inputs, add them together, and put the sume
    /// // in the 9th slot on the tape.
    /// let tape: Vec<i64> = vec![3,5,3,6,1101,0,0,9,99,0];
    /// let mut comp = intcode::IntcodeComp::new(tape);
    /// 
    /// // Push one of the inputs proactively.
    /// comp.push_input(5);
    /// 
    /// // Run the computer.
    /// comp.start();
    /// while *comp.state() != intcode::IntcodeState::Finished {
    ///     match comp.state() {
    ///         // Push more input reactively.
    ///         intcode::IntcodeState::NeedsInput => {
    ///             comp.push_input(10);
    ///             comp.start();
    ///         },
    ///         intcode::IntcodeState::Err(s) => panic!("Unexpected error: {}", s),
    ///         _ => (),
    ///     }
    /// }
    /// 
    /// // Verify results
    /// assert_eq!(*comp.state(), intcode::IntcodeState::Finished);
    /// assert_eq!(comp.get(9), 15);
    /// ```
    pub fn push_input(&mut self, i: i64) {
        self.inputs.push_back(i);
    }

    /// Retrieve output from the computer's output queue.  This function pops a
    /// single output value from the front of the queue and returns it.
    /// 
    /// # Example
    /// 
    /// ```
    /// // This tape will take two inputs and output both their sum and their
    /// // product.
    /// let tape: Vec<i64> = vec![3,5,3,6,1101,0,0,13,2,5,6,14,99,0,0];
    /// let mut comp = intcode::IntcodeComp::new(tape);
    /// 
    /// // Push the inputs proactively.
    /// comp.push_input(5);
    /// comp.push_input(6);
    /// 
    /// // Run the computer.
    /// comp.start();
    /// 
    /// // Verify results
    /// assert_eq!(*comp.state(), intcode::IntcodeState::Finished);
    /// assert_eq!(comp.get(13), 11);
    /// assert_eq!(comp.get(14), 30);
    /// ```
    pub fn pop_output(&mut self) -> Option<i64> {
        self.outputs.pop_front()
    }

    /// Returns the current state of the computer.
    pub fn state(&self) -> &IntcodeState {
        &self.state
    }

    /// Private function used to read and execute the tape's next instruction.
    fn execute_one(&mut self) {
        let op = self.get(self.head) % 100;
        self.mode = self.get(self.head) / 100;
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
            9 => self.rel_adjust(),
            _ => panic!("Unsupported opcode!  Current computer state: opcode {}, head {}, mode {}",
                        op, self.head, self.mode),
        }
    }

    /// Helper function that reads a parameter pointed to by the computer's
    /// head, and returns its value based on the current parameter mode.
    fn get_param(&mut self, is_output: bool) -> i64 {
        let m = self.mode % 10;
        self.mode /= 10;
        match m {
            // Positional: get the value from the specified position.
            0 => {
                let pos = self.get(self.head);
                self.head += 1;
                if is_output { return pos; }
                return self.get(pos as usize);
            },
            // Immediate: use this value directly.
            1 => {
                if is_output {
                    panic!("Mode 1 not supported for output params!  Current \
                            computer state: m {}, head {}, mode {}",
                            m, self.head, self.mode)
                }
                let val = self.get(self.head);
                self.head += 1;
                return val;
            },
            // Relative: add this value to self.rel_base and use the value at
            // that position.
            2 => {
                let pos = self.rel_base + self.get(self.head);
                self.head += 1;
                if is_output { return pos; }
                return self.get(pos as usize);
            }
            _ => panic!("Unsupported mode!  Current computer state: m {}, head {}, mode {}",
                        m, self.head, self.mode),
        }
    }

    /// Implementation of the add operation.
    /// 
    /// # Example
    /// 
    /// ```
    /// let tape: Vec<i64> = vec![1,5,6,4,0,49,50];
    /// let mut comp = intcode::IntcodeComp::new(tape);
    /// comp.start();
    /// assert_eq!(*comp.state(), intcode::IntcodeState::Finished);
    /// assert_eq!(comp.get(4), 99);
    /// ```
    fn add(&mut self) {
        let x = self.get_param(false);
        let y = self.get_param(false);
        let pos: usize = self.get_param(true) as usize;
        self.set(pos, x + y);
    }

    /// Implementation of the multiply operation.
    /// 
    /// # Example
    /// 
    /// ```
    /// let tape: Vec<i64> = vec![2,5,6,4,0,3,33];
    /// let mut comp = intcode::IntcodeComp::new(tape);
    /// comp.start();
    /// assert_eq!(*comp.state(), intcode::IntcodeState::Finished);
    /// assert_eq!(comp.get(4), 99);
    /// ```
    fn mult(&mut self) {
        let x = self.get_param(false);
        let y = self.get_param(false);
        let pos: usize = self.get_param(true) as usize;
        self.set(pos, x * y);
    }

    /// Implementation of the input operation.
    /// 
    /// # Example
    /// 
    /// ```
    /// let tape: Vec<i64> = vec![3,2,0];
    /// let mut comp = intcode::IntcodeComp::new(tape);
    /// comp.start();
    /// comp.push_input(99);
    /// comp.start();
    /// assert_eq!(*comp.state(), intcode::IntcodeState::Finished);
    /// assert_eq!(comp.get(2), 99);
    /// ```
    fn input(&mut self) {
        let maybe_input = self.inputs.pop_front();
        match maybe_input {
            Some(input) => {
                let pos: usize = self.get_param(true) as usize;
                self.set(pos, input);
            },
            None => self.state = IntcodeState::NeedsInput,
        }
    }

    /// Implementation of the output operation.
    /// 
    /// # Example
    /// 
    /// ```
    /// let tape: Vec<i64> = vec![4,3,99,50];
    /// let mut comp = intcode::IntcodeComp::new(tape);
    /// comp.start();
    /// assert_eq!(*comp.state(), intcode::IntcodeState::Finished);
    /// assert_eq!(comp.pop_output(), Some(50));
    /// ```
    fn output(&mut self) {
        let out = self.get_param(false);
        self.outputs.push_back(out);
    }

    /// Implementation of the jump-if-true operation.
    /// 
    /// # Example
    /// 
    /// ```
    /// let tape: Vec<i64> = vec![5,1,8,99,5,9,9,99,4,0];
    /// let mut comp = intcode::IntcodeComp::new(tape);
    /// comp.start();
    /// assert_eq!(*comp.state(), intcode::IntcodeState::Finished);
    /// ```
    fn jump_if_true(&mut self) {
        let x = self.get_param(false);
        let y = self.get_param(false);
        if x != 0 {
            self.head = y as usize;
        }
    }

    /// Implementation of the jump-if-false operation.
    /// 
    /// # Example
    /// 
    /// ```
    /// let tape: Vec<i64> = vec![6,5,8,99,6,0,9,99,4,0];
    /// let mut comp = intcode::IntcodeComp::new(tape);
    /// comp.start();
    /// assert_eq!(*comp.state(), intcode::IntcodeState::Finished);
    /// ```
    fn jump_if_false(&mut self) {
        let x = self.get_param(false);
        let y = self.get_param(false);
        if x == 0 {
            self.head = y as usize;
        }
    }

    /// Implementation of the less-than operation.
    /// 
    /// # Example
    /// 
    /// ```
    /// let tape: Vec<i64> = vec![7,1,2,0,7,2,1,4,99];
    /// let mut comp = intcode::IntcodeComp::new(tape);
    /// comp.start();
    /// assert_eq!(*comp.state(), intcode::IntcodeState::Finished);
    /// assert_eq!(comp.get(0), 1);
    /// assert_eq!(comp.get(4), 0);
    /// ```
    fn less_than(&mut self) {
        let x = self.get_param(false);
        let y = self.get_param(false);
        let pos: usize = self.get_param(true) as usize;
        if x < y {
            self.set(pos, 1);
        } else {
            self.set(pos, 0);
        }
    }

    /// Implementation of the equals operation.
    /// 
    /// # Example
    /// 
    /// ```
    /// let tape: Vec<i64> = vec![8,9,10,0,8,10,11,4,99,1,2,2];
    /// let mut comp = intcode::IntcodeComp::new(tape);
    /// comp.start();
    /// assert_eq!(*comp.state(), intcode::IntcodeState::Finished);
    /// assert_eq!(comp.get(0), 0);
    /// assert_eq!(comp.get(4), 1);
    /// ```
    fn equals(&mut self) {
        let x = self.get_param(false);
        let y = self.get_param(false);
        let pos: usize = self.get_param(true) as usize;
        if x == y {
            self.set(pos, 1);
        } else {
            self.set(pos, 0);
        }
    }

    /// Implementation of the relative-adjust operation.
    /// 
    /// # Example
    /// 
    /// ```
    /// // This tape does 3 similar addition operations.  But each operation
    /// // uses relative mode for the first argument, and we adjust the
    /// // relative base between each operation.
    /// let tape: Vec<i64> = vec![201,1,2,0,109,4,201,1,2,6,109,-3,201,1,2,12,99];
    /// let mut comp = intcode::IntcodeComp::new(tape);
    /// comp.start();
    /// assert_eq!(*comp.state(), intcode::IntcodeState::Finished);
    /// assert_eq!(comp.get(0), 3);
    /// assert_eq!(comp.get(6), 6);
    /// assert_eq!(comp.get(12), 4);
    /// ```
    fn rel_adjust(&mut self) {
        let x = self.get_param(false);
        self.rel_base += x;
    }
}

/// Helper function to help convert a comma-delimited string of integers into
/// a vector of integers (to be passed to an IntcodeComp as tape).
/// 
/// # Example
/// 
/// ```
/// let line = "1,5,6,0,99,2,2";
/// let tape = intcode::to_tape(&line);
/// let mut comp = intcode::IntcodeComp::new(tape);
/// comp.start();
/// assert_eq!(*comp.state(), intcode::IntcodeState::Finished);
/// assert_eq!(comp.get(0), 4);
/// ```
pub fn to_tape(line: &str) -> Vec<i64> {
    line.split(",").map(|p| p.parse::<i64>().unwrap()).collect()
}