/**
 * [467] Unique Substrings in Wraparound String
 *
 * We define the string s to be the infinite wraparound string of "abcdefghijklmnopqrstuvwxyz", so s will look like this:
 * 
 * 	"...zabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcd....".
 * 
 * Given a string p, return the number of unique non-empty substrings of p are present in s.
 *  
 * Example 1:
 * 
 * Input: p = "a"
 * Output: 1
 * Explanation: Only the substring "a" of p is in s.
 * 
 * Example 2:
 * 
 * Input: p = "cac"
 * Output: 2
 * Explanation: There are two substrings ("a", "c") of p in s.
 * 
 * Example 3:
 * 
 * Input: p = "zab"
 * Output: 6
 * Explanation: There are six substrings ("z", "a", "b", "za", "ab", and "zab") of p in s.
 * 
 *  
 * Constraints:
 * 
 * 	1 <= p.length <= 10^5
 * 	p consists of lowercase English letters.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/unique-substrings-in-wraparound-string/
// discuss: https://leetcode.com/problems/unique-substrings-in-wraparound-string/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn find_substring_in_wrapround_string(p: String) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_467() {
    }
}
