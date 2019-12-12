fn checksum(line: &str, layer_size: usize) -> i32 {
    let mut rem: &str = line;
    let mut fewest_zeroes = -1;
    let mut saved_ones = 0;
    let mut saved_twos = 0;
    while rem.len() != 0 {
        let (first, second) = rem.split_at(layer_size);
        let mut zeroes = 0;
        let mut ones = 0;
        let mut twos = 0;
        for c in first.chars() {
            match c {
                '0' => zeroes += 1,
                '1' => ones += 1,
                '2' => twos += 1,
                _ => (),
            }
        }
        if fewest_zeroes == -1 || zeroes < fewest_zeroes {
            fewest_zeroes = zeroes;
            saved_ones = ones;
            saved_twos = twos;
        }
        rem = second;
    }
    return saved_ones * saved_twos;
}

fn merge(line: &str, layer_size: usize) -> String {
    let mut rem: &str = line;
    let mut merged = "2".repeat(layer_size);
    while rem.len() != 0 {
        let (first, second) = rem.split_at(layer_size);
        let new_merged = merged.chars().zip(first.chars()).map(|(a, b)| {
            if a == '2' {
                return b;
            }
            a
        }).collect();
        merged = new_merged;
        rem = second;
    }
    return merged;
}

fn main() {
    let line = std::fs::read_to_string("day-eight/input.txt").expect("file not found");
    let width = 25;
    let height = 6;
    let layer_size = width*height;

    let checksum = checksum(&line, layer_size);

    println!("Checksum {}", checksum);

    let image = merge(&line, layer_size);

    let mut i = 1;
    for c in image.chars() {
        // Trust me, this makes it a lot easier to read the message.
        match c {
            '0' => print!("  "),
            '1' => print!("# "),
            _ => (),
        }
        if i == width {
            println!("");
            i = 0;
        }
        i += 1;
    }
}
