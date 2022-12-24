pub struct Solution;

struct F(i32, i32);
struct P(i32);
struct State(F, P);

const MOD: i32 = 1_000_000_007;
impl Solution {
    // Fn+1 = Fn + Fn-1 + Pn + P`n
    // Fn|[*]   or  Fn-1|[**]   or  Pn|[*]   or    |[**]
    // __|[*]       ____|[**]       _|[**]       P`n|[*]
    // Pn+1 = Fn-1 + P`n
    // Fn-1|[**]  or    |[**]
    // ____|[*]       P`n|
    // P`n+1 = Fn-1 + Pn
    // Fn-1|[*]   or   Pn|
    // ____|[**]       _|[**]
    pub fn num_tilings(n: i32) -> i32 {
        (0..n).fold(State(F(1, 0), P(0)), |State(f, p), _el| {
            State(F(((f.0 + f.1) % MOD + (2*p.0) % MOD) % MOD, f.0), P((f.1 + p.0) % MOD))
        }).0.0
    }    
}

#[cfg(test)]
mod tests {
    use test_case::test_case;
    use super::Solution;

    #[test_case(1, 1)]
    #[test_case(3, 5)]
    #[test_case(30, 312342182)]
    fn solve(n: i32, expected: i32) {
        assert_eq!(expected, Solution::num_tilings(n));
    }
}