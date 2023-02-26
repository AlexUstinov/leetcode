pub struct Solution;

impl Solution {
    pub fn sort_array(mut nums: Vec<i32>) -> Vec<i32> {
        fn sort(nums: &mut [i32]) {
            let len = nums.len();
            if len < 2 {
                return;
            }
            let pivot = nums[(len-1) / 2];
            let (mut l, mut r) = (0, len+1);
            let mid = loop {
                l += 1;
                while nums[l-1] < pivot {
                    l += 1;
                }
                r -= 1;
                while nums[r-1] > pivot {
                    r -= 1;
                }
                if r <= l {
                    break r-1;
                }
                nums.swap(l-1, r-1);
            };
            sort(&mut nums[0..=mid]);
            sort(&mut nums[(mid+1)..]);
        }
        sort(&mut nums[..]);
        nums
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;
    use super::Solution;

    #[test_case(vec![1])]
    #[test_case(vec![2,1])]
    #[test_case(vec![1,2])]
    #[test_case(vec![1,2,3])]
    #[test_case(vec![1,3,2])]
    #[test_case(vec![3,1,2])]
    #[test_case(vec![2,1,3])]
    #[test_case(vec![2,3,1])]
    #[test_case(vec![3,2,1])]
    #[test_case(vec![5,1,1,2,0,0])]
    fn solve(nums: Vec<i32>) {
        let mut expected = nums.clone();
        expected.sort_unstable();
        assert_eq!(expected, Solution::sort_array(nums));
    }
}