pub fn check_luhn(cc_number: &str) -> bool {
    let mut sum: i32 = 0;
    let mut double: bool = false;
    let mut digits: i32 = 0;

    // Reverse de credit or check account number
    for char in cc_number.chars().rev() {
        if let Some(digit) = char.to_digit(10) {
            digits += 1;
            if double {
                let double_digit: i32 = (digit * 2) as i32;
                sum += if double_digit > 9 {
                    double_digit - 9
                } else {
                    double_digit
                };
            } else {
                sum += digit as i32;
            }
            double = !double;
        } else if char.is_whitespace() {
            continue;
        } else {
            return false;
        }
    }
    digits >= 2 && sum % 10 == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_cc_number() {
        assert!(check_luhn("4263 9826 4026 9299"));
        assert!(check_luhn("4539 3195 0343 6467"));
        assert!(check_luhn("7992 7398 713"));
    }

    #[test]
    fn test_invalid_cc_number() {
        assert!(!check_luhn("4223 9826 4026 9299"));
        assert!(!check_luhn("4539 3195 0343 6476"));
        assert!(!check_luhn("8273 1232 7352 0569"));
    }

    #[test]
    fn test_non_digit_cc_number() {
        assert!(!check_luhn("foo 0 o"));
        assert!(!check_luhn("foo"));
    }

    #[test]
    fn test_empty_cc_number() {
        assert!(!check_luhn(""));
        assert!(!check_luhn(" "));
        assert!(!check_luhn("  "));
    }
}
