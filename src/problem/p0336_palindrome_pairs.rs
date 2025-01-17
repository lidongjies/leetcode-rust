/**
 * [336] Palindrome Pairs
 *
 * Given a list of unique words, return all the pairs of the distinct indices (i, j) in the given list, so that the concatenation of the two words words[i] + words[j] is a palindrome.
 *  
 * Example 1:
 * 
 * Input: words = ["abcd","dcba","lls","s","sssll"]
 * Output: [[0,1],[1,0],[3,2],[2,4]]
 * Explanation: The palindromes are ["dcbaabcd","abcddcba","slls","llssssll"]
 * 
 * Example 2:
 * 
 * Input: words = ["bat","tab","cat"]
 * Output: [[0,1],[1,0]]
 * Explanation: The palindromes are ["battab","tabbat"]
 * 
 * Example 3:
 * 
 * Input: words = ["a",""]
 * Output: [[0,1],[1,0]]
 * 
 *  
 * Constraints:
 * 
 * 	1 <= words.length <= 5000
 * 	0 <= words[i].length <= 300
 * 	words[i] consists of lower-case English letters.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/palindrome-pairs/
// discuss: https://leetcode.com/problems/palindrome-pairs/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn palindrome_pairs(words: Vec<String>) -> Vec<Vec<i32>> {
        vec![]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_336() {
    }
}
