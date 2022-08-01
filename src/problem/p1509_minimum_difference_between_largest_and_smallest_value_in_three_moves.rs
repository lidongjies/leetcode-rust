/**
 * [1509] Minimum Difference Between Largest and Smallest Value in Three Moves
 *
 * You are given an integer array nums. In one move, you can choose one element of nums and change it by any value.
 * Return the minimum difference between the largest and smallest value of nums after performing at most three moves.
 *  
 * Example 1:
 * 
 * Input: nums = [5,3,2,4]
 * Output: 0
 * Explanation: Change the array [5,3,2,4] to [2,2,2,2].
 * The difference between the maximum and minimum is 2-2 = 0.
 * 
 * Example 2:
 * 
 * Input: nums = [1,5,0,10,14]
 * Output: 1
 * Explanation: Change the array [1,5,0,10,14] to [1,1,0,1,1]. 
 * The difference between the maximum and minimum is 1-0 = 1.
 * 
 *  
 * Constraints:
 * 
 * 	1 <= nums.length <= 10^5
 * 	-10^9 <= nums[i] <= 10^9
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-difference-between-largest-and-smallest-value-in-three-moves/
// discuss: https://leetcode.com/problems/minimum-difference-between-largest-and-smallest-value-in-three-moves/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn min_difference(nums: Vec<i32>) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1509() {
    }
}
