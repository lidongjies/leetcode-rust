/**
 * [678] Valid Parenthesis String
 *
 * Given a string s containing only three types of characters: '(', ')' and '*', return true if s is valid.
 * The following rules define a valid string:
 * 
 * 	Any left parenthesis '(' must have a corresponding right parenthesis ')'.
 * 	Any right parenthesis ')' must have a corresponding left parenthesis '('.
 * 	Left parenthesis '(' must go before the corresponding right parenthesis ')'.
 * 	'*' could be treated as a single right parenthesis ')' or a single left parenthesis '(' or an empty string "".
 * 
 *  
 * Example 1:
 * Input: s = "()"
 * Output: true
 * Example 2:
 * Input: s = "(*)"
 * Output: true
 * Example 3:
 * Input: s = "(*))"
 * Output: true
 *  
 * Constraints:
 * 
 * 	1 <= s.length <= 100
 * 	s[i] is '(', ')' or '*'.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/valid-parenthesis-string/
// discuss: https://leetcode.com/problems/valid-parenthesis-string/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn check_valid_string(s: String) -> bool {
        false
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_678() {
    }
}
