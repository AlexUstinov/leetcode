pub struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let (mut num, mut result) = (x, 0);
        x == 0 || (x > 0 && x % 10 != 0 && loop {
            if num <= result {
                break num == result || num == result / 10
            }
            result *= 10;
            result += num % 10;
            num /= 10;
        })
    }
}

#[cfg(test)]
mod test {
    use test_case::test_case;
    use super::Solution;

    #[test_case(0, true)]
    #[test_case(10, false)]
    #[test_case(121, true)]
    #[test_case(-121, false)]
    fn solve(x:i32, expected: bool) {
        assert_eq!(expected, Solution::is_palindrome(x))
    }

}