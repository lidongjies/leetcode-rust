/**
 * [1871] Jump Game VII
 *
 * You are given a 0-indexed binary string s and two integers minJump and maxJump. In the beginning, you are standing at index 0, which is equal to '0'. You can move from index i to index j if the following conditions are fulfilled:
 * 
 * 	i + minJump <= j <= min(i + maxJump, s.length - 1), and
 * 	s[j] == '0'.
 * 
 * Return true if you can reach index s.length - 1 in s, or false otherwise.
 *  
 * Example 1:
 * 
 * Input: s = "<u>0</u>11<u>0</u>1<u>0</u>", minJump = 2, maxJump = 3
 * Output: true
 * Explanation:
 * In the first step, move from index 0 to index 3. 
 * In the second step, move from index 3 to index 5.
 * 
 * Example 2:
 * 
 * Input: s = "01101110", minJump = 2, maxJump = 3
 * Output: false
 * 
 *  
 * Constraints:
 * 
 * 	2 <= s.length <= 10^5
 * 	s[i] is either '0' or '1'.
 * 	s[0] == '0'
 * 	1 <= minJump <= maxJump < s.length
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/jump-game-vii/
// discuss: https://leetcode.com/problems/jump-game-vii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn can_reach(s: String, min_jump: i32, max_jump: i32) -> bool {
        false
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1871() {
    }
}
