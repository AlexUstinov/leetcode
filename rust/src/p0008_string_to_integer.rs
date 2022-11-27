pub struct Solution;

enum State {
    LeadingSymbols,
    Digits
}

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut sign = 1;
        let mut result = 0i32;
        let mut state = State::LeadingSymbols;

        fn add_digit(result: &i32, sign: &i32, digit: &u8) -> i32 {
            if let Some(new_result) = result.checked_mul(10) {
                let digit = (digit - b'0') as i32;
                if let Some(new_result) = new_result.checked_add(digit * sign) {
                    return new_result;
                }
            }
            if *sign > 0 { i32::MAX } else { i32::MIN }
        }

        for c in s.as_bytes() {
            if let i32::MAX | i32::MIN = result {
                break;
            }
            match (c, &state) {
                (b' ', State::LeadingSymbols) => continue,
                (b'+', State::LeadingSymbols) => {
                    state = State::Digits;
                    continue;
                },
                (b'-', State::LeadingSymbols) => {
                    state = State::Digits;
                    sign = -1;
                    continue;
                },
                (b'0'..=b'9', State::LeadingSymbols) => {
                    state = State::Digits;
                    result = add_digit(&result, &sign, c);
                },
                (b'0'..=b'9', State::Digits) => {
                    result = add_digit(&result, &sign, c);
                },
                _ => break,
            }
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::Solution;
    use test_case::test_case;

    #[test_case(String::from(" "), 0)]
    #[test_case(String::from("abc"), 0)]
    #[test_case(String::from("123"), 123)]
    #[test_case(String::from("-124"), -124)]
    #[test_case(String::from("  +125abc"), 125)]
    #[test_case(String::from("3000000000"), i32::MAX)]
    #[test_case(String::from("-3000000000"), i32::MIN)]
    fn solve(s: String, expected: i32) {
        assert_eq!(expected, Solution::my_atoi(s));
    }
}
