use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn to_fuel(weight: &i32) -> i32 {
    (weight / 3) - 2
}

fn expand(module: &i32) -> i32 {
    let mut total = 0;
    let mut add = to_fuel(module);
    while add > 0 {
        total += add;
        add = to_fuel(&add);
    }
    total
}

fn main() {
    let f = File::open("day-one/input.txt").expect("file not found");
    let reader = BufReader::new(&f);
    let result = reader.lines()
                        .map(|l| l.unwrap())
                        .map(|s| s.parse::<i32>().unwrap())
                        .fold(0, |acc, input| acc + expand(&input));

    println!("Sum of total fuel requirements: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_fuel() {
        let cases = vec![
            (12, 2),
            (14, 2),
            (1969, 654),
            (100756, 33583),
        ];

        for (input, expected) in cases.iter() {
            assert_eq!(to_fuel(&input), *expected);
        }
    }

    #[test]
    fn test_expand() {
        let cases = vec![
            (12, 2),
            (14, 2),
            (1969, 966),
            (100756, 50346),
        ];

        for (input, expected) in cases.iter() {
            assert_eq!(expand(&input), *expected);
        }
    }
}