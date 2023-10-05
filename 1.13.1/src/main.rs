// TODO: remove this when you're done with your implementation.
#![allow(unused_variables, dead_code)]

pub fn luhn(cc_number: &str) -> bool {
    let mut sum: i32 = 0;
    let mut chars: i32 = 0;
    for c in cc_number.chars().rev() {
        if c == ' ' {
            continue;
        }

        let num_option: Option<u32> = c.to_digit(10);
        let mut num: i32;
        match num_option {
            None => return false,
            Some(num_i) => num = num_i as i32,
        }

        chars += 1;

        if chars % 2 == 0 {
            num = num * 2;
            if num > 9 {
                num = num % 10 + num / 10;
            }
        }

        sum = sum + num;
    }

    return chars > 1 && sum % 10 == 0;
}

#[test]
fn test_non_digit_cc_number() {
    assert!(!luhn("foo"));
    assert!(!luhn("foo 0 0"));
}

#[test]
fn test_empty_cc_number() {
    assert!(!luhn(""));
    assert!(!luhn(" "));
    assert!(!luhn("  "));
    assert!(!luhn("    "));
}

#[test]
fn test_single_digit_cc_number() {
    assert!(!luhn("0"));
}

#[test]
fn test_two_digit_cc_number() {
    assert!(luhn(" 0 0 "));
}

#[test]
fn test_valid_cc_number() {
    assert!(luhn("4263 9826 4026 9299"));
    assert!(luhn("4539 3195 0343 6467"));
    assert!(luhn("7992 7398 713"));
}

#[test]
fn test_invalid_cc_number() {
    assert!(!luhn("4223 9826 4026 9299"));
    assert!(!luhn("4539 3195 0343 6476"));
    assert!(!luhn("8273 1232 7352 0569"));
}

#[allow(dead_code)]
fn main() {}
