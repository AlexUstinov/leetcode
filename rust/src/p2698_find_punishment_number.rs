pub struct Solution;

use std::collections::VecDeque;

impl Solution {
    pub fn punishment_number(n: i32) -> i32 {
        fn is_punishment_number(i: i32) -> bool {
            fn solve(digits: &[i32], sum: i32, num: i32) -> bool {
                if digits.is_empty() {
                    return sum == num;
                }
                if digits[0] == 0 {
                    return solve(&digits[1..], sum, num);
                }
                for idx in 1..=digits.len() {
                    let mut val = 0;
                    for d in digits[0..idx].iter().copied() {
                        val = val * 10 + d;
                    }
                    if sum + val > num {
                        break;
                    }
                    if solve(&digits[idx..], sum + val, num) {
                        return true;
                    }
                }
                false
            }
            let mut digits = VecDeque::new();
            let mut num = i * i;
            while num > 0 {
                digits.push_front(num % 10);
                num /= 10;
            }

            solve(digits.make_contiguous(), 0, i)
        }

        (1..=n).filter(|&i| is_punishment_number(i)).map(|i| i * i).sum()
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;
    use super::Solution;

    #[test_case(2, 1)]
    #[test_case(10, 182)]
    fn solve(n: i32, expected: i32) {
        assert_eq!(expected, Solution::punishment_number(n));
    }
}