use std::num;

/**
 * [004] median of two sorted arrays
 *
 * 给定两个大小分别为 m 和 n 的正序（从小到大）数组 nums1 和 nums2。请你找出并返回这两个正序数组的 中位数 。
 *
 * 算法的时间复杂度应该为 O(log (m+n)) 。
 *
 *
 * 示例 1：
 *
 * 输入：nums1 = [1,3], nums2 = [2]
 * 输出：2.00000
 * 解释：合并数组 = [1,2,3] ，中位数 2
 * 示例 2：
 *
 * 输入：nums1 = [1,2], nums2 = [3,4]
 * 输出：2.50000
 * 解释：合并数组 = [1,2,3,4] ，中位数 (2 + 3) / 2 = 2.5
 *
 * 提示：
 *
 * nums1.length == m
 * nums2.length == n
 * 0 <= m <= 1000
 * 0 <= n <= 1000
 * 1 <= m + n <= 2000
 * -106 <= nums1[i], nums2[i] <= 106
 */
use std::cmp;

struct Solution;

/**
 * 思路：
 * 1. 归并排序，取中位数，时间复杂度 O(m+n)，空间复杂度 O(m+n)
 * 2. 找到两个数组的中位数的位置，时间复杂度 O(m+n)，空间复杂度 O(1)
 * 3. 二分法找到两个数组的中位数的位置，时间复杂度 O(log(m+n))，空间复杂度 O(1)
 * 4. 中位数的统计学意义，将两个数组拆分，时间复杂度 O(log(min(m+n)))，空间复杂度 O(1)
 */
impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let length_1 = nums1.len();
        let length_2 = nums2.len();
        let total_length = length_1 + length_2;
        if (total_length % 2 == 1) {
            let medium = total_length / 2;
            Solution::find_median_sorted_arrays_by_binary_search(&nums1, &nums2, medium + 1) as f64
        } else {
            let medium = total_length / 2;
            let left = medium - 1;
            let right = medium;
            ((Solution::find_median_sorted_arrays_by_binary_search(&nums1, &nums2, left + 1)
                + Solution::find_median_sorted_arrays_by_binary_search(&nums1, &nums2, right + 1))
                as f64)
                / 2.0
        }
    }

    fn find_median_sorted_arrays_by_binary_search(nums1: &[i32], nums2: &[i32], k: usize) -> i32 {
        let length1 = nums1.len();
        let length2 = nums2.len();
        let mut index1 = 0;
        let mut index2 = 0;
        let mut k = k;
        let mut new_index1 = 0;
        let mut new_index2 = 0;
        loop {
            println!(
                "index1:{} index2:{} newIndex1:{} newIndex2:{}, k:{}",
                index1, index2, new_index1, new_index2, k
            );
            if (index1 == nums1.len()) {
                return nums2[index2 + k - 1];
            }
            if (index2 == nums2.len()) {
                return nums1[index1 + k - 1];
            }
            if (k == 1) {
                return cmp::min(nums1[index1], nums2[index2]);
            }
            let half = k / 2;
            new_index1 = cmp::min(index1 + half, length1) - 1;
            new_index2 = cmp::min(index2 + half, length2) - 1;
            if (nums1[new_index1] <= nums2[new_index2]) {
                k -= new_index1 - index1 + 1;
                index1 = new_index1 + 1;
            } else {
                k -= new_index2 - index2 + 1;
                index2 = new_index2 + 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_004() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 3], vec![2]),
            2.0
        );

        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]),
            2.5
        )
    }
}
