/**
 * [1417] reformat the string
 * https://leetcode.cn/problems/reformat-the-string/
 */
pub struct Solution {}

impl Solution {
    pub fn reformat(s: String) -> String {
        let total_count = s.len() as i32;
        let digit_count = s.chars().filter(|c| c.is_digit(10)).count() as i32;
        let alphabetic_count = total_count - digit_count;
        let diff = digit_count - alphabetic_count;
        if diff.abs() > 1 {
            return "".into();
        }

        let mut i = 0;
        let mut j = 0;
        if digit_count >= alphabetic_count {
            j += 1;
        } else {
            i += 1;
        }
        let mut new_chars = vec!['_'; total_count as usize];
        s.chars().for_each(|c| {
            if c.is_digit(10) {
                new_chars[i] = c;
                i += 2;
            } else {
                new_chars[j] = c;
                j += 2;
            }
        });
        let mut result = String::new();
        new_chars.iter().for_each(|c| result.push(*c));
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1417() {
        assert_eq!(
            Solution::reformat("a0b1c2".to_string()),
            "0a1b2c".to_string()
        );
        assert_eq!(Solution::reformat("leetcode".to_string()), "".to_string());
        assert_eq!(Solution::reformat("1229857369".to_string()), "".to_string());
        assert_eq!(
            Solution::reformat("covid2019".to_string()),
            "c2o0v1i9d".to_string()
        );
        assert_eq!(Solution::reformat("ab123".to_string()), "1a2b3".to_string());
    }
}
