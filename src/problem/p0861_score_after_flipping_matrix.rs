/**
 * [861] Score After Flipping Matrix
 *
 * You are given an m x n binary matrix grid.
 * A move consists of choosing any row or column and toggling each value in that row or column (i.e., changing all 0's to 1's, and all 1's to 0's).
 * Every row of the matrix is interpreted as a binary number, and the score of the matrix is the sum of these numbers.
 * Return the highest possible score after making any number of moves (including zero moves).
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/07/23/lc-toogle1.jpg" style="width: 500px; height: 299px;" />
 * Input: grid = [[0,0,1,1],[1,0,1,0],[1,1,0,0]]
 * Output: 39
 * Explanation: 0b1111 + 0b1001 + 0b1111 = 15 + 9 + 15 = 39
 * 
 * Example 2:
 * 
 * Input: grid = [[0]]
 * Output: 1
 * 
 *  
 * Constraints:
 * 
 * 	m == grid.length
 * 	n == grid[i].length
 * 	1 <= m, n <= 20
 * 	grid[i][j] is either 0 or 1.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/score-after-flipping-matrix/
// discuss: https://leetcode.com/problems/score-after-flipping-matrix/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn matrix_score(grid: Vec<Vec<i32>>) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_861() {
    }
}
