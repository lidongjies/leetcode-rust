/**
 * [1296] Divide Array in Sets of K Consecutive Numbers
 *
 * Given an array of integers nums and a positive integer k, check whether it is possible to divide this array into sets of k consecutive numbers.
 * Return true if it is possible. Otherwise, return false.
 *  
 * Example 1:
 * 
 * Input: nums = [1,2,3,3,4,4,5,6], k = 4
 * Output: true
 * Explanation: Array can be divided into [1,2,3,4] and [3,4,5,6].
 * 
 * Example 2:
 * 
 * Input: nums = [3,2,1,2,3,4,3,4,5,9,10,11], k = 3
 * Output: true
 * Explanation: Array can be divided into [1,2,3] , [2,3,4] , [3,4,5] and [9,10,11].
 * 
 * Example 3:
 * 
 * Input: nums = [1,2,3,4], k = 3
 * Output: false
 * Explanation: Each array should be divided in subarrays of size 3.
 * 
 *  
 * Constraints:
 * 
 * 	1 <= k <= nums.length <= 10^5
 * 	1 <= nums[i] <= 10^9
 * 
 *  
 * Note: This question is the same as 846: <a href="https://leetcode.com/problems/hand-of-straights/" target="_blank">https://leetcode.com/problems/hand-of-straights/</a>
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/divide-array-in-sets-of-k-consecutive-numbers/
// discuss: https://leetcode.com/problems/divide-array-in-sets-of-k-consecutive-numbers/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn is_possible_divide(nums: Vec<i32>, k: i32) -> bool {
        false
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1296() {
    }
}
