extern crate regex;
extern crate mod_exp;

use regex::Regex;
use mod_exp::mod_exp;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Shuffle {
    Reverse,
    Cut(i128),
    Deal(i128)
}

fn modulus(a: i128, b: i128) -> i128 {
    mod_exp(a, 1, b)
}

fn inv(a: i128, b: i128) -> i128 {
    mod_exp(a, b-2, b)
}

fn compress(shuffles: &Vec<Shuffle>, deck_size: i128) -> (i128, i128) {
    shuffles.iter().fold((1, 0), |(a, b), &shuffle| {
        match shuffle {
            Shuffle::Reverse => (modulus(-a, deck_size), modulus(-b - 1, deck_size)),
            Shuffle::Cut(val) => (modulus(a, deck_size), modulus(b + val, deck_size)),
            Shuffle::Deal(val) => (modulus(a*val, deck_size), modulus(b*val, deck_size)),
        }
    })
}

fn track_card(a: i128, b: i128, card: i128, deck_size: i128, times: i128) -> i128 {
    let new_a = mod_exp(a, times, deck_size);
    let new_b = modulus(b * (new_a - 1) * inv(a-1, deck_size), deck_size);
    modulus(new_a*card + new_b, deck_size)
}

fn track_pos(a: i128, b: i128, pos: i128, deck_size: i128, times: i128) -> i128 {
    let new_a = mod_exp(a, times, deck_size);
    let tmp = modulus((new_a - 1) * inv(a-1, deck_size), deck_size);
    let new_b = modulus(b * tmp, deck_size);
    modulus((pos - new_b) * inv(new_a, deck_size), deck_size)
}

fn main() {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("input.txt");
    let input = std::fs::read_to_string(&path).expect("file not found");

    let stack_re = Regex::new(r"deal into new stack").unwrap();
    let cut_re = Regex::new(r"cut (-?\d*)").unwrap();
    let inc_re = Regex::new(r"deal with increment (\d*)").unwrap();

    let shuffles: Vec<Shuffle> = input.lines().map(|line| {
        if stack_re.is_match(line) {
            Shuffle::Reverse
        } else if cut_re.is_match(line) {
            let caps = cut_re.captures(line).unwrap();
            let val = -(caps[1].parse::<i128>().unwrap());
            Shuffle::Cut(val)
        } else if inc_re.is_match(line) {
            let caps = inc_re.captures(line).unwrap();
            let val = caps[1].parse::<i128>().unwrap();
            Shuffle::Deal(val)
        } else {
            panic!("Unexpected input: {}", line);
        }
    }).collect();

    // Part 1
    {
        let size = 10007_i128;
        let card = 2019_i128;
        let times = 1_i128;
        let (a, b) = compress(&shuffles, size);

        println!("deck size: {}", size);
        println!("number of shuffles: {}", times);
        println!("a: {}, b: {}", a, b);
        println!("position of card {}: {}", card, track_card(a, b, card, size, times));
    }
    println!("");
    // Part 2
    {
        let size = 119315717514047_i128;
        let pos = 2020_i128;
        let times = 101741582076661_i128;
        let (a, b) = compress(&shuffles, size);

        println!("deck size: {}", size);
        println!("number of shuffles: {}", times);
        println!("a: {}, b: {}", a, b);
        println!("card at position {}: {}", pos, track_pos(a, b, pos, size, times));
    }
}