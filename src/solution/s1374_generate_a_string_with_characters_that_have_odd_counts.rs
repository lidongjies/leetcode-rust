/**
 * [1374] generate_a_string_with_characters_that_have_odd_counts
 * https://leetcode.cn/problems/generate-a-string-with-characters-that-have-odd-counts/
 */

struct Solution;

impl Solution {
    pub fn generate_the_string(n: i32) -> String {
        if (n & 1) == 1 {
            (0..n).map(|_| 'a').collect::<String>()
        } else {
            let mut strings = (0..n - 1).map(|_| 'b').collect::<String>();
            strings.push('a');
            strings
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1374() {
        assert_eq!(Solution::generate_the_string(1), "a");
        assert_eq!(Solution::generate_the_string(2), "ba");
        assert_eq!(Solution::generate_the_string(3), "aaa");
    }
}
