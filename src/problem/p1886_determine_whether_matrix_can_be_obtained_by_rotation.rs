/**
 * [1886] Determine Whether Matrix Can Be Obtained By Rotation
 *
 * Given two n x n binary matrices mat and target, return true if it is possible to make mat equal to target by rotating mat in 90-degree increments, or false otherwise.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/05/20/grid3.png" style="width: 301px; height: 121px;" />
 * Input: mat = [[0,1],[1,0]], target = [[1,0],[0,1]]
 * Output: true
 * Explanation: We can rotate mat 90 degrees clockwise to make mat equal target.
 * 
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/05/20/grid4.png" style="width: 301px; height: 121px;" />
 * Input: mat = [[0,1],[1,1]], target = [[1,0],[0,1]]
 * Output: false
 * Explanation: It is impossible to make mat equal to target by rotating mat.
 * 
 * Example 3:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/05/26/grid4.png" style="width: 661px; height: 184px;" />
 * Input: mat = [[0,0,0],[0,1,0],[1,1,1]], target = [[1,1,1],[0,1,0],[0,0,0]]
 * Output: true
 * Explanation: We can rotate mat 90 degrees clockwise two times to make mat equal target.
 * 
 *  
 * Constraints:
 * 
 * 	n == mat.length == target.length
 * 	n == mat[i].length == target[i].length
 * 	1 <= n <= 10
 * 	mat[i][j] and target[i][j] are either 0 or 1.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/determine-whether-matrix-can-be-obtained-by-rotation/
// discuss: https://leetcode.com/problems/determine-whether-matrix-can-be-obtained-by-rotation/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn find_rotation(mat: Vec<Vec<i32>>, target: Vec<Vec<i32>>) -> bool {
        false
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1886() {
    }
}
