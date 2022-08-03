/**
 * [907] Sum of Subarray Minimums
 *
 * Given an array of integers arr, find the sum of min(b), where b ranges over every (contiguous) subarray of arr. Since the answer may be large, return the answer modulo 10^9 + 7.
 *  
 * Example 1:
 * 
 * Input: arr = [3,1,2,4]
 * Output: 17
 * Explanation: 
 * Subarrays are [3], [1], [2], [4], [3,1], [1,2], [2,4], [3,1,2], [1,2,4], [3,1,2,4]. 
 * Minimums are 3, 1, 2, 4, 1, 1, 2, 1, 1, 1.
 * Sum is 17.
 * 
 * Example 2:
 * 
 * Input: arr = [11,81,94,43,3]
 * Output: 444
 * 
 *  
 * Constraints:
 * 
 * 	1 <= arr.length <= 3 * 10^4
 * 	1 <= arr[i] <= 3 * 10^4
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/sum-of-subarray-minimums/
// discuss: https://leetcode.com/problems/sum-of-subarray-minimums/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn sum_subarray_mins(arr: Vec<i32>) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_907() {
    }
}
