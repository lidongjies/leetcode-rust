/**
 * [420] Strong Password Checker
 *
 * A password is considered strong if the below conditions are all met:
 * 
 * 	It has at least 6 characters and at most 20 characters.
 * 	It contains at least one lowercase letter, at least one uppercase letter, and at least one digit.
 * 	It does not contain three repeating characters in a row (i.e., "...aaa..." is weak, but "...aa...a..." is strong, assuming other conditions are met).
 * 
 * Given a string password, return the minimum number of steps required to make password strong. if password is already strong, return 0.
 * In one step, you can:
 * 
 * 	Insert one character to password,
 * 	Delete one character from password, or
 * 	Replace one character of password with another character.
 * 
 *  
 * Example 1:
 * Input: password = "a"
 * Output: 5
 * Example 2:
 * Input: password = "aA1"
 * Output: 3
 * Example 3:
 * Input: password = "1337C0d3"
 * Output: 0
 *  
 * Constraints:
 * 
 * 	1 <= password.length <= 50
 * 	password consists of letters, digits, dot '.' or exclamation mark '!'.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/strong-password-checker/
// discuss: https://leetcode.com/problems/strong-password-checker/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn strong_password_checker(password: String) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_420() {
    }
}
