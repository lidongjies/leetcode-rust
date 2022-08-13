/**
 * [768] max chunks to make sorted II
 * https://leetcode.cn/problems/max-chunks-to-make-sorted-ii/
 */

pub struct Solution {}

impl Solution {
    pub fn max_chunks_to_sorted(arr: Vec<i32>) -> i32 {
        let mut strategy = 0_i32;
        let mut sorted_arr = arr.clone();
        sorted_arr.sort();
        let mut sum_arr = 0;
        let mut sum_arr_sorted = 0;
        for i in 0..arr.len() {
            sum_arr += arr[i];
            sum_arr_sorted += sorted_arr[i];
            if sum_arr == sum_arr_sorted {
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
    fn test_0768() {
        assert_eq!(Solution::max_chunks_to_sorted(vec![5, 4, 3, 2, 1]), 1);
        assert_eq!(Solution::max_chunks_to_sorted(vec![2, 1, 3, 4, 4]), 4);
    }
}
