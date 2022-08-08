use std::collections::HashMap;

/**
 * [0008] string to &integer atoi
 * https://leetcode.cn/problems/string-to-integer-atoi/
 */

pub struct Automaton {
    state: String,
    states: HashMap<&'static str, [&'static str; 4]>,
    pub ans: i64,
    pub sign: i8,
}

impl Automaton {
    pub fn new() -> Automaton {
        let automaton_states = HashMap::from([
            ("start", ["start", "signed", "in_number", "end"]),
            ("signed", ["end", "end", "in_number", "end"]),
            ("in_number", ["end", "end", "in_number", "end"]),
            ("end", ["end", "end", "end", "end"]),
        ]);
        Automaton {
            state: String::from("start"),
            states: automaton_states,
            ans: 0,
            sign: 1,
        }
    }

    pub fn get_state(&self, c: char) -> usize {
        if c.is_whitespace() {
            return 0;
        }
        if c.eq(&'-') || c.eq(&'+') {
            return 1;
        }
        if c.is_digit(10) {
            return 2;
        }
        3
    }

    pub fn get(&mut self, c: char) {
        let state = self.get_state(c);
        let states = self.states.get(&self.state[..]).unwrap();
        self.state = states[state].to_string();
        if self.state.eq("in_number") {
            self.ans = self.ans * 10 + c.to_digit(10).unwrap() as i64;
            self.ans = if self.sign == 1 {
                self.ans.min(i32::MAX as i64)
            } else {
                self.ans.min(-(i32::MIN as i64))
            }
        } else if self.state.eq("signed") {
            self.sign = if c.eq(&'+') { 1 } else { -1 }
        }
    }
}

pub struct Solution {}

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut auto_man = Automaton::new();
        for c in s.chars() {
            auto_man.get(c);
        }
        return (auto_man.ans * auto_man.sign as i64) as i32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_0008() {
        assert_eq!(Solution::my_atoi("".to_string()), 0);
        assert_eq!(Solution::my_atoi("   ".to_string()), 0);
        assert_eq!(Solution::my_atoi("   -".to_string()), 0);
        assert_eq!(Solution::my_atoi("   -a".to_string()), 0);
        assert_eq!(Solution::my_atoi("21474836460".to_string()), 2147483647);
        assert_eq!(Solution::my_atoi("   -2147483648".to_string()), -2147483648);
        assert_eq!(Solution::my_atoi("   +2147483647".to_string()), 2147483647);
        assert_eq!(Solution::my_atoi("42".to_string()), 42);
        assert_eq!(Solution::my_atoi("-42".to_string()), -42);
        assert_eq!(Solution::my_atoi("4193 with words".to_string()), 4193);
    }
}
