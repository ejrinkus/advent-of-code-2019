// Pt. 1 solution, runs in-place.
fn run_fft(data: &mut Vec<i32>, phases: u8) {
    let base_pattern: Vec<i32> = vec![0, 1, 0, -1];
    let mut patterns: Vec<Vec<i32>> = vec![base_pattern.clone()];

    for _ in 0..phases {
        let mut output: Vec<i32> = Vec::with_capacity(data.len());
        for i in 0..data.len() {
            let mut sum = 0;
            if i >= patterns.len() {
                let mut new_pattern = vec![];
                for p in &base_pattern {
                    for _ in 0..i+1 {
                        new_pattern.push(*p);
                    }
                }
                patterns.push(new_pattern);
            }
            let pattern = &patterns[i];
            for i in 0..data.len() {
                let pattern_i = (i+1) % pattern.len();
                sum += data[i] * pattern[pattern_i];
            }
            output.push((sum % 10).abs());
        }
        *data = output;
    }
}

// Pt. 2 solution.  Only works if operating on the 2nd half of the original
// input.
fn run_shortcut_fft(data: &mut Vec<i32>, phases: u8) {
    for _ in 0..phases {
        for i in (0..data.len()-1).rev() {
            data[i] = (data[i+1] + data[i]) % 10;
        }
    }
}

fn main() {
    let line = std::fs::read_to_string("day-sixteen/input.txt").expect("file not found");
    let input: Vec<i32> = line.as_bytes().iter().map(|x| (x-48) as i32).collect();

    let mut input_pt_one = input.clone();
    run_fft(&mut input_pt_one, 100);
    for i in 0..8 {
        print!("{}", input_pt_one[i]);
    }
    println!("");

    // Take the first 7 values in the input and combine them into a number.
    let input_len: usize = input.len() as usize;
    let offset: usize = input[0..7].iter().fold(0 ,|acc, x| (acc*10)+x) as usize;
    let repeats: usize = 10000;
    let full_len: usize = input_len * repeats;
    if offset < full_len / 2 {
        println!("well shit, our solution isn't generalized to work on the front half :(");
        return;
    } else if offset >= full_len {
        println!("!! {}, {} !!", offset, full_len);
        panic!("wat the fack");
    }

    // Just the number of digits after offset.
    let trimmed_len: usize = full_len - offset;
    // The number of times we will repeat the full input within trimmed_len.
    let trimmed_repeats: usize = trimmed_len / input_len;
    // The number of additional digits we need to take from the end of the
    // input, and prepend onto the new input.
    let partial_input_len: usize = trimmed_len % input_len;
    println!("original input length: {}", input_len);
    println!("length after {} repeats: {}", repeats, full_len);
    println!("offset: {}", offset);
    println!("trimmed length: {}", trimmed_len);
    println!("number of full repeats in trimmed input: {}", trimmed_repeats);
    println!("partial input length: {}", partial_input_len);

    // Build the trimmed input by first grabbing the last partial_input_len
    // digits from the original input.
    let prefix_start: usize = input.len() - partial_input_len;
    let prefix = &input[prefix_start..input.len()];
    let mut trimmed_input: Vec<i32> = vec![0; trimmed_len];
    trimmed_input[0..partial_input_len].copy_from_slice(prefix);

    // Now fill the rest of the trimmed input with repeats of the original.
    let mut i = partial_input_len;
    while i < trimmed_input.len() {
        let end = i + input_len;
        trimmed_input[i..end].copy_from_slice(&input);
        i = end;
    }

    // Now we can finally run part 2.
    run_shortcut_fft(&mut trimmed_input, 100);
    for i in 0..8 {
        print!("{}", trimmed_input[i]);
    }
    println!("");
}
