/**
 * [581] Shortest Unsorted Continuous Subarray
 *
 * Given an integer array nums, you need to find one continuous subarray that if you only sort this subarray in ascending order, then the whole array will be sorted in ascending order.
 * Return the shortest such subarray and output its length.
 *  
 * Example 1:
 * 
 * Input: nums = [2,6,4,8,10,9,15]
 * Output: 5
 * Explanation: You need to sort [6, 4, 8, 10, 9] in ascending order to make the whole array sorted in ascending order.
 * 
 * Example 2:
 * 
 * Input: nums = [1,2,3,4]
 * Output: 0
 * 
 * Example 3:
 * 
 * Input: nums = [1]
 * Output: 0
 * 
 *  
 * Constraints:
 * 
 * 	1 <= nums.length <= 10^4
 * 	-10^5 <= nums[i] <= 10^5
 * 
 *  
 * Follow up: Can you solve it in O(n) time complexity?
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/shortest-unsorted-continuous-subarray/
// discuss: https://leetcode.com/problems/shortest-unsorted-continuous-subarray/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn find_unsorted_subarray(nums: Vec<i32>) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_581() {
    }
}
