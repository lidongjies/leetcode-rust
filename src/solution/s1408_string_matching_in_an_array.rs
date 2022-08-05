/**
 * [1408] string_matching_in_an_array
 * https://leetcode.cn/problems/string-matching-in-an-array/
 * 
 * 给你一个字符串数组 words ，数组中的每个字符串都可以看作是一个单词。请你按 任意 顺序返回 words 中是其他单词的子字符串的所有单词。
 * 如果你可以删除 words[j] 最左侧和/或最右侧的若干字符得到 word[i] ，那么字符串 words[i] 就是 words[j] 的一个子字符串。
 * 
 * 示例 1：
 * 
 * 输入：words = ["mass","as","hero","superhero"]
 * 输出：["as","hero"]
 * 解释："as" 是 "mass" 的子字符串，"hero" 是 "superhero" 的子字符串。
 * ["hero","as"] 也是有效的答案。
 * 示例 2：
 * 
 * 输入：words = ["leetcode","et","code"]
 * 输出：["et","code"]
 * 解释："et" 和 "code" 都是 "leetcode" 的子字符串。
 * 示例 3：
 * 
 * 输入：words = ["blue","green","bu"]
 * 输出：[]
 * 
 * 
 * 提示：
 * 
 * 1 <= words.length <= 100
 * 1 <= words[i].length <= 30
 * words[i] 仅包含小写英文字母。
 * 题目数据 保证 每个 words[i] 都是独一无二的。
 */

pub struct Solution {}

impl Solution {
    // 时间复杂度 O(n^2 * L2)，空间复杂度 O(n)
    // 题解里只提到了用 KMP 可以把复杂度优化到 O(n^2L)，如果再进一步把 KMP 换成 AC 自动机就可以达到线性复杂度了。
    pub fn string_matching(words: Vec<String>) -> Vec<String> {
        let mut arr = Vec::new();
        for (i, word1) in words.iter().enumerate() {
            for (j, word2) in words.iter().enumerate() {
                if i != j && word2.contains(word1) {
                    arr.push(word1);
                    break;
                }
            }
        }
        arr.iter().map(|s| s.to_string()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1408() {
        // assert_eq!(Solution::string_matching(vec!["mass","as","hero","superhero"]), vec!["as", "hero"]);
        // assert_eq!(Solution::string_matching(vec!["leetcode","et","code"], vec!["et", "code"]));
    }
}