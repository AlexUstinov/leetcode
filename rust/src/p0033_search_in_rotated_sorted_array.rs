pub struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let n = nums.len();
        let (mut l, mut r) = (0, n);
        while l < r {
            let mid = (l + r) / 2;
            if nums[mid] > nums[l] {
                l = mid + 1;
                if l < n && nums[mid] > nums[l] {
                    break;
                }
            }
            else if mid > l && nums[mid-1] > nums[mid] {
                l = mid;
                break;
            }
            else {
                r = mid;
            }
        }
        if l==n {
            nums.binary_search(&target).ok().map(|idx| idx as i32).unwrap_or(-1)
        }
        else {
            nums[0..l].binary_search(&target).ok()
                .or_else(|| nums[l..].binary_search(&target).ok().map(|idx| idx + l))
                .map_or(-1, |idx| idx as i32)
        }
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;
    use super::Solution;

    #[test_case(vec![1,3], 0, -1)]
    #[test_case(vec![3,1], 3, 0)]
    #[test_case(vec![4,5,6,7,0,1,2], 0, 4)]
    fn solve(nums: Vec<i32>, target: i32, expected: i32) {
        assert_eq!(expected, Solution::search(nums, target));
    }

}