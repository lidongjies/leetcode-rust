/**
 * [130] Surrounded Regions
 *
 * Given an m x n matrix board containing 'X' and 'O', capture all regions that are 4-directionally surrounded by 'X'.
 * A region is captured by flipping all 'O's into 'X's in that surrounded region.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/02/19/xogrid.jpg" style="width: 550px; height: 237px;" />
 * Input: board = [["X","X","X","X"],["X","O","O","X"],["X","X","O","X"],["X","O","X","X"]]
 * Output: [["X","X","X","X"],["X","X","X","X"],["X","X","X","X"],["X","O","X","X"]]
 * Explanation: Notice that an 'O' should not be flipped if:
 * - It is on the border, or
 * - It is adjacent to an 'O' that should not be flipped.
 * The bottom 'O' is on the border, so it is not flipped.
 * The other three 'O' form a surrounded region, so they are flipped.
 * 
 * Example 2:
 * 
 * Input: board = [["X"]]
 * Output: [["X"]]
 * 
 *  
 * Constraints:
 * 
 * 	m == board.length
 * 	n == board[i].length
 * 	1 <= m, n <= 200
 * 	board[i][j] is 'X' or 'O'.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/surrounded-regions/
// discuss: https://leetcode.com/problems/surrounded-regions/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_130() {
    }
}
