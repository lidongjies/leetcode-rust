/**
 * [1422] maximum score after aplitting a string
 * https://leetcode.cn/problems/maximum-score-after-splitting-a-string/
 */

pub struct Solution {}

impl Solution {
    pub fn max_score(s: String) -> i32 {
        let mut count_1 = 0;
        let chars = s.chars().collect::<Vec<char>>();
        // 左右字串非空
        // 左侧至少有一个，左侧第一个字符为0才得分
        if chars[0] == '0' {
            count_1 += 1;
        }
        // 计算右子串的得分
        for index in 1..chars.len() {
            if chars[index] == '1' {
                count_1 += 1;
            }
        }
        // 遍历所有拆分位置
        let mut max_score = count_1;
        for index in 1..chars.len() - 1 {
            if chars[index] == '1' {
                count_1 -= 1;
            } else {
                count_1 += 1;
            }
            max_score = max_score.max(count_1);
        }
        max_score as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1422() {
        assert_eq!(Solution::max_score("011101".to_string()), 5);
        assert_eq!(Solution::max_score("00111".to_string()), 5);
        assert_eq!(Solution::max_score("1111".to_string()), 3);
    }
}
