fn to_digits(mut i: u32) -> Vec<u32> {
    let mut digits: Vec<u32> = Vec::new();
    loop {
        digits.insert(0, i % 10);
        i /= 10;
        if i <= 0 { break; }
    }
    digits
}

fn is_valid(digits: &Vec<u32>) -> bool {
    if digits.len() <= 1 { return false; }

    let mut found_doubles = false;
    let mut val: u32 = digits[0];
    let mut count: u32 = 1;
    for i in 1..digits.len() {
        if digits[i] < digits[i-1] {
            return false;
        }
        if digits[i] == val {
            count += 1;
            continue;
        }
        if count == 2 {
            found_doubles = true;
        }
        val = digits[i];
        count = 1;
    }
    return found_doubles || count == 2;
}

fn main() {
    let mut count = 0;
    for combo in 172851..675869 {
        let digits: Vec<u32> = to_digits(combo);
        if is_valid(&digits) { count += 1; }
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

    #[test]
    fn test_is_valid() {
        let cases = [
            (vec![1,1,1,1,1,1], false),
            (vec![2,2,3,4,5,0], false),
            (vec![1,2,3,7,8,9], false),
            (vec![1,1,2,2,3,3], true),
            (vec![1,2,3,4,4,4], false),
            (vec![1,1,1,1,2,2], true),
        ];
        for (input, expected) in cases.iter() {
            let got = is_valid(input);
            assert_eq!(got, *expected, "For input {:?}: expected {}, got {}", input, expected, got);
        }
    }
}