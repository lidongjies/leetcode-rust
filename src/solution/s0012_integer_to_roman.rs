/**
 * [0012] integer to roman
 * https://leetcode.cn/problems/integer-to-roman/
 */

pub struct Solution {}

impl Solution {
    pub fn int_to_roman_my(num: i32) -> String {
        let symbols: Vec<Vec<(i32, &str)>> = vec![
            vec![(1000, "M")],
            vec![(900, "CM"), (500, "D"), (400, "CD"), (100, "C")],
            vec![(90, "XC"), (50, "L"), (40, "XL"), (10, "X")],
            vec![(9, "IX"), (5, "V"), (4, "IV"), (1, "I")],
        ];
        let mut roman = Vec::new();
        let mut number = num;
        // integer log10 是 nightly api，无法通过 leetcode 编译
        // let count = num.log10() + 1;
        let count = num.to_string().len();
        for i in (4 - count)..4 {
            let s = symbols.get(i).unwrap();
            for (radix, symbol) in s {
                let count = number / radix;
                number %= radix;
                roman.push(symbol.repeat(count as usize));
            }
        }
        roman.join("")
    }

    pub fn int_to_roman(num: i32) -> String {
        let symbols: Vec<(i32, &str)> = vec![
            (1000, "M"),
            (900, "CM"),
            (500, "D"),
            (400, "CD"),
            (100, "C"),
            (90, "XC"),
            (50, "L"),
            (40, "XL"),
            (10, "X"),
            (9, "IX"),
            (5, "V"),
            (4, "IV"),
            (1, "I"),
        ];
        let mut number = num;
        let mut roman = String::new();
        for (radix, symbol) in symbols {
            while number >= radix {
                number -= radix;
                roman += symbol;
            }
            if number == 0 {
                break;
            }
        }
        roman
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0012() {
        assert_eq!(Solution::int_to_roman(3), "III".to_string());
        assert_eq!(Solution::int_to_roman(4), "IV".to_string());
        assert_eq!(Solution::int_to_roman(9), "IX".to_string());
        assert_eq!(Solution::int_to_roman(58), "LVIII".to_string());
        assert_eq!(Solution::int_to_roman(3), "III".to_string());
    }
}
