use std::cmp;
#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let (nums1, len1, nums2, len2) = match (nums1.len(), nums2.len()) {
            (len1, len2) if len1 > len2 => (nums2, len2, nums1, len1),
            (len1, len2) => (nums1, len1, nums2, len2)
        };
        let len = len1 + len2;
        let half_len = len >> 1;
        let (mut low, mut high) = (0usize, len1);

        loop {
            let cut1 = (low + high) >> 1;
            let cut2 = half_len - cut1;

            let (ul, ur) = (match cut1 { 0 => i32::MIN, _ => nums1[cut1-1] }, match cut1 { cut1 if cut1 < len1 => nums1[cut1], _ => i32::MAX });
            let (bl, br) = (match cut2 { 0 => i32::MIN, _ => nums2[cut2-1] }, match cut2 { cut2 if cut2 < len2 => nums2[cut2], _ => i32::MAX });
            if ul > br {
                high = cut1 - 1; // We've already seen the (cut1-1)'th element (the value of ul), so excluding it
            }
            else if bl > ur {
                low = cut1 + 1;  // We've already seen the cut1'th element (the value of ur), so excluding it
            }
            else {
                break match len {
                    len if (len & 1) > 0 => cmp::min(ur, br) as f64,
                    _ => (cmp::max(ul, bl) + cmp::min(ur, br)) as f64 / 2f64
                };
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![], vec![2], 2f64)]
    #[test_case(vec![2], vec![], 2f64)]
    #[test_case(vec![2], vec![2], 2f64)]
    #[test_case(vec![], vec![2, 2], 2f64)]
    #[test_case(vec![1, 3 ], vec![ 2 ], 2f64)]
    #[test_case(vec![1, 2 ], vec![ 3, 4 ], 2.5f64)]
    #[test_case(vec![1, 4 ], vec![ 2, 3 ], 2.5f64)]
    #[test_case(vec![3, 3 ], vec![ 1, 1, 1 ], 1f64)]
    #[test_case(vec![3, 3, 3, 3, 3, 3, 3, 3 ], vec![ 1, 1, 1, 1, 1 ], 3f64)]
    #[test_case(vec![3, 3, 3, 3, 3, 3, 3, 3 ], vec![ 1, 1, 1, 1, 1, 1 ], 3f64)]
    #[test_case(vec![3, 3, 3, 3, 3, 3, 3, 3 ], vec![ 1, 1, 1, 1, 1, 1, 1 ], 3f64)]
    #[test_case(vec![1, 1, 1, 1, 1, 1, 1, 1 ], vec![ 3, 3, 3, 3, 3 ], 1f64)]
    #[test_case(vec![1, 1, 1, 1, 1, 1, 1, 1 ], vec![ 3, 3, 3, 3, 3, 3 ], 1f64)]
    #[test_case(vec![1, 1, 1, 1, 1, 1, 1, 1 ], vec![ 3, 3, 3, 3, 3, 3, 3 ], 1f64)]
    #[test_case(vec![1, 10 ], vec![ 2, 3, 4, 5, 6, 6, 7, 7, 8, 8, 9, 9 ], 6.5f64)]
    #[test_case(vec![1, 2, 3, 5, 6 ], vec![ 4 ], 3.5f64)]
    fn solve(nums1: Vec<i32>, nums2: Vec<i32>, expected: f64) {
        assert_eq!(expected, Solution::find_median_sorted_arrays(nums1, nums2));
    }
}
