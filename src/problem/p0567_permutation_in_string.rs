/**
 * [567] Permutation in String
 *
 * Given two strings s1 and s2, return true if s2 contains a permutation of s1, or false otherwise.
 * In other words, return true if one of s1's permutations is the substring of s2.
 *  
 * Example 1:
 * 
 * Input: s1 = "ab", s2 = "eidbaooo"
 * Output: true
 * Explanation: s2 contains one permutation of s1 ("ba").
 * 
 * Example 2:
 * 
 * Input: s1 = "ab", s2 = "eidboaoo"
 * Output: false
 * 
 *  
 * Constraints:
 * 
 * 	1 <= s1.length, s2.length <= 10^4
 * 	s1 and s2 consist of lowercase English letters.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/permutation-in-string/
// discuss: https://leetcode.com/problems/permutation-in-string/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        false
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_567() {
    }
}
