fn main() {
    let line = std::fs::read_to_string("day-eight/input.txt").expect("file not found");
    let width = 25;
    let height = 6;
    let layer_size = width*height;

    let mut rem: &str = &line;
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

    println!("{}", saved_ones * saved_twos);
}
