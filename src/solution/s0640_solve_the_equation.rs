use std::num;

/**
 * [640] solve the equation
 * https://leetcode.cn/problems/solve-the-equation/
 */

pub struct Solution {}

impl Solution {
    pub fn solve_equation(equation: String) -> String {
        let mut factor = 0;
        let mut value = 0;
        let mut sign1 = 1_i32;
        let mut index = 0;
        let len = equation.len();
        let chars: Vec<char> = equation.chars().collect();

        while index < len {
            // 判断是否为等号，更改默认系数
            if chars[index] == '=' {
                sign1 = -1;
                index += 1;
                continue;
            }

            let mut sign2 = sign1;
            let mut number = 0_i32;
            let mut valid = false;
            // 判断是否为正负号，设置当前项的系数
            if chars[index] == '-' || chars[index] == '+' {
                sign2 = if chars[index] == '-' { -sign1 } else { sign1 };
                index += 1;
            }

            // 判断是否为合法数字，并进行累加
            while index < len && chars[index].is_digit(10) {
                number = number * 10 + chars[index].to_digit(10).unwrap() as i32;
                index += 1;
                valid = true;
            }

            // 判断是否为变量x，并设置对 factor 累加
            if index < len && chars[index] == 'x' {
                factor += if valid { number * sign2 } else { sign2 };
                index += 1;
            } else {
                value += sign2 * number;
            }
        }
        if factor == 0 {
            if value == 0 {
                "Infinite solutions".to_string()
            } else {
                "No solution".to_string()
            }
        } else {
            format!("x={:}", (-value / factor))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_640() {
        assert_eq!(
            Solution::solve_equation("x+5-3+x=6+x-2".to_string()),
            "x=2".to_string()
        )
    }
}
