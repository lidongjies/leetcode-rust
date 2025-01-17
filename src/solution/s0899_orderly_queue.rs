/**
 * [899] 有序队列
 *
 * 给定一个字符串 s 和一个整数 k 。你可以从 s 的前 k 个字母中选择一个，并把它加到字符串的末尾。
 * 返回 在应用上述步骤的任意数量的移动后，字典上最小的字符串 。
 *
 *
 * 示例 1：
 * 输入：s = "cba", k = 1
 * 输出："acb"
 * 解释：
 * 在第一步中，我们将第一个字符（“c”）移动到最后，获得字符串 “bac”。
 * 在第二步中，我们将第一个字符（“b”）移动到最后，获得最终结果 “acb”。
 * 示例 2：
 *
 * 输入：s = "baaca", k = 3
 * 输出："aaabc"
 * 解释：
 * 在第一步中，我们将第一个字符（“b”）移动到最后，获得字符串 “aacab”。
 * 在第二步中，我们将第三个字符（“c”）移动到最后，获得最终结果 “aaabc”。
 *
 * 提示：
 *
 * 1 <= k <= S.length <= 1000
 * s 只由小写字母组成。
*/

struct Solution {}

impl Solution {
    pub fn orderly_queue(s: String, k: i32) -> String {
        if (k == 1) {
            let mut min_dic = s.clone();
            for i in 0..s.len() {
                // let new_dic = String::from(&s[i..]) + &s[0..i];
                let new_dic = format!("{}{}", &s[i..], &s[0..i]);
                if (new_dic < min_dic) {
                    min_dic = new_dic;
                }
            }
            return min_dic;
        }

        unsafe {
            let mut vec = s.into_bytes();
            vec.sort();
            return String::from_utf8(vec).unwrap();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_899() {
        assert_eq!(
            Solution::orderly_queue(String::from("cba"), 1),
            String::from("acb")
        );
        assert_eq!(
            Solution::orderly_queue(String::from("baaca"), 3),
            String::from("aaabc")
        )
    }
}
