pub struct Solution;

impl Solution {
    pub fn closest_primes(left: i32, right: i32) -> Vec<i32> {
        let mut bit_vec = vec![true; (right + 1) as usize];
        let mut div = 2;
        bit_vec[0] = false;
        bit_vec[1] = false;
        while div <= right {
            let mut mult = 2;
            let mut idx = div * mult;
            while idx < right + 1 {
                bit_vec[idx as usize] = false;
                mult +=1;
                idx = div * mult;
            }
            div += 1;
            while div < right + 1 {
                if bit_vec[div as usize] {
                    break;
                }
                div += 1;
            }
        }
        let mut primes = Vec::new();
        for num in left..=right {
            if bit_vec[num as usize] {
                primes.push(num);
            }
        }
        if primes.len() < 2 {
            return vec![-1, -1];
        }
        let mut min = i32::MAX;
        let mut min_idx = 0;
        for idx in 1usize..primes.len() {
            let min_candidate = primes[idx] - primes[idx-1];
            if min_candidate < min {
                min_idx = idx;
                min = min_candidate;
            }
        }
        vec![primes[min_idx-1], primes[min_idx]]
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;
    use super::Solution;

    #[test_case(10, 19, vec![11, 13])]
    fn solve(left: i32, right: i32, expected: Vec<i32>) {
        assert_eq!(expected, Solution::closest_primes(left, right));
    }
}