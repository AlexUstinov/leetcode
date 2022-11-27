pub struct Solution;

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut x = x;
        let mut result = 0i32;
        loop {
            if let Some(new_result) = result.checked_mul(10) {
                let q = x / 10;
                let r = x - q * 10;
                if let Some(new_result) = new_result.checked_add(r) {
                    if q == 0 {
                        break new_result;
                    }
                    x = q;
                    result = new_result;
                    continue;
                }
            }
            break 0;
        }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;
    use test_case::test_case;

    #[test_case(123, 321)]
    #[test_case(-321, -123)]
    #[test_case(i32::MAX, 0)]
    #[test_case(i32::MIN, 0)]
    fn solve(x: i32, expected: i32) {
        assert_eq!(expected, Solution::reverse(x));
    }
}
