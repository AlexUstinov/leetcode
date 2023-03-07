pub struct Solution;

impl Solution {
    pub fn divide(mut dividend: i32, mut divisor: i32) -> i32 {
        let is_negative = (dividend & i32::MIN) ^ (divisor & i32::MIN) != 0;
        if dividend & i32::MIN == 0 {
            dividend = !dividend + 1;
        }
        if divisor & i32::MIN == 0 {
            divisor = !divisor + 1;
        }
        let max_divisor = divisor;
        while divisor >= i32::MIN >> 1 && divisor << 1 >= dividend {
            divisor <<= 1;
        }
        let mut result = 0;
        while divisor <= max_divisor {
            result <<= 1;
            if dividend <= divisor {
                result -= 1;
                dividend -= divisor;
            }
            divisor = (divisor + 1) >> 1;
        }
        if is_negative {
            result
        }
        else {
            match !result {
                i32::MAX => i32::MAX,
                r => r + 1
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;
    use super::Solution;
    
    #[test_case(10, 3)]
    #[test_case(6, -3)]
    #[test_case(7, -3)]
    #[test_case(i32::MIN, 1)]
    #[test_case(i32::MIN, 0-1)]
    #[test_case(i32::MAX, 0-1)]
    #[test_case(i32::MAX, 1)]
    fn solve(dividend: i32, divisor: i32) {
        let mut expected = dividend as i64 / divisor as i64;
        if expected > i32::MAX as i64 {
            expected = i32::MAX as i64;
        }
        assert_eq!(expected as i32, Solution::divide(dividend, divisor));
    }

}