use std::num;

/**
* [0006] zigzag_conversion
* https://leetcode.cn/problems/zigzag-conversion/
*
* 将一个给定字符串 s 根据给定的行数 numRows ，以从上往下、从左到右进行 Z 字形排列。

比如输入字符串为 "PAYPALISHIRING" 行数为 3 时，排列如下：

P   A   H   N
A P L S I I G
Y   I   R
之后，你的输出需要从左往右逐行读取，产生出一个新的字符串，比如："PAHNAPLSIIGYIR"。

请你实现这个将字符串进行指定行数变换的函数：

string convert(string s, int numRows);


示例 1：

输入：s = "PAYPALISHIRING", numRows = 3
输出："PAHNAPLSIIGYIR"
示例 2：
输入：s = "PAYPALISHIRING", numRows = 4
输出："PINALSIGYAHRPI"
解释：
P     I    N
A   L S  I G
Y A   H R
P     I
示例 3：

输入：s = "A", numRows = 1
输出："A"


提示：

1 <= s.length <= 1000
s 由英文字母（小写和大写）、',' 和 '.' 组成
1 <= numRows <= 1000
*/

pub struct Solution {}

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let num_rows = num_rows as usize;
        let total_count = s.len();
        if num_rows == 1 || num_rows >= total_count {
            return s;
        }
        let count_pre_t = num_rows * 2 - 2; // 每个周期的字母数量
        let num_cols = (total_count + count_pre_t - 1) / count_pre_t * (num_rows - 1);
        let mut x = 0;
        let mut y = 0;
        let mut matrix = vec![Vec::new(); num_rows];
        for (i, b) in s.bytes().enumerate() {
            matrix[x].push(b);
            if i % count_pre_t < num_rows - 1 {
                x += 1
            } else {
                x = (x + num_rows - 1) % num_rows;
            }
        }
        std::str::from_utf8(&matrix.concat()).unwrap().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0006() {
        assert_eq!(
            Solution::convert(String::from("PAYPALISHIRING"), 3),
            String::from("PAHNAPLSIIGYIR")
        );

        assert_eq!(
            Solution::convert(String::from("PAYPALISHIRING"), 4),
            String::from("PINALSIGYAHRPI")
        );
    }
}
