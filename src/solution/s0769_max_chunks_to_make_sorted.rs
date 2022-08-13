/**
 * [769] max chunks to make sorted
 * https://leetcode.cn/problems/max-chunks-to-make-sorted/
 */

pub struct Solution {}

impl Solution {
    pub fn max_chunks_to_sorted(arr: Vec<i32>) -> i32 {
        let mut strategy = 0_i32;
        let mut max = 0_i32;
        for (index, number) in arr.iter().enumerate() {
            max = max.max(*number);
            if (index as i32) == max {
                strategy += 1;
            }
        }
        strategy
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0769() {
        assert_eq!(Solution::max_chunks_to_sorted(vec![4, 3, 2, 1, 0]), 1);
        assert_eq!(Solution::max_chunks_to_sorted(vec![1, 0, 2, 3, 4]), 4);
    }
}
