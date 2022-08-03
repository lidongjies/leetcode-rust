/**
 * [368] Largest Divisible Subset
 *
 * Given a set of distinct positive integers nums, return the largest subset answer such that every pair (answer[i], answer[j]) of elements in this subset satisfies:
 * 
 * 	answer[i] % answer[j] == 0, or
 * 	answer[j] % answer[i] == 0
 * 
 * If there are multiple solutions, return any of them.
 *  
 * Example 1:
 * 
 * Input: nums = [1,2,3]
 * Output: [1,2]
 * Explanation: [1,3] is also accepted.
 * 
 * Example 2:
 * 
 * Input: nums = [1,2,4,8]
 * Output: [1,2,4,8]
 * 
 *  
 * Constraints:
 * 
 * 	1 <= nums.length <= 1000
 * 	1 <= nums[i] <= 2 * 10^9
 * 	All the integers in nums are unique.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/largest-divisible-subset/
// discuss: https://leetcode.com/problems/largest-divisible-subset/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn largest_divisible_subset(nums: Vec<i32>) -> Vec<i32> {
        vec![]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_368() {
    }
}