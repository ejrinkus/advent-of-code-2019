fn to_digits(mut i: u32) -> Vec<u32> {
    let mut digits: Vec<u32> = Vec::new();
    loop {
        digits.insert(0, i % 10);
        i /= 10;
        if i <= 0 { break; }
    }
    digits
}

fn main() {
    let mut count = 0;
    for combo in 172851..675869 {
        let digits: Vec<u32> = to_digits(combo);
        let mut is_decreasing = false;
        let mut found_doubles = false;
        for i in 1..digits.len() {
            if digits[i] < digits[i-1] {
                is_decreasing = true;
                break;
            }
            if digits[i] == digits[i-1] {
                found_doubles = true;
            }
        }
        if !is_decreasing && found_doubles { count += 1; }
    }
    println!("Found {} possible passwords", count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_digits() {
        let cases = [
            (0, vec![0]),
            (2, vec![2]),
            (10, vec![1,0]),
            (599387, vec![5,9,9,3,8,7]),
        ];
        for (input, expected) in cases.iter() {
            let digits = to_digits(*input);
            assert_eq!(digits.len(), expected.len());
            let diff = digits.iter().zip(expected.iter()).filter(|&(a, b)| a != b).count();
            assert_eq!(diff, 0);
        }
    }
}