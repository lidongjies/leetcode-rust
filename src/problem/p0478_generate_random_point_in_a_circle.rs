/**
 * [478] Generate Random Point in a Circle
 *
 * Given the radius and the position of the center of a circle, implement the function randPoint which generates a uniform random point inside the circle.
 * Implement the Solution class:
 * 
 * 	Solution(double radius, double x_center, double y_center) initializes the object with the radius of the circle radius and the position of the center (x_center, y_center).
 * 	randPoint() returns a random point inside the circle. A point on the circumference of the circle is considered to be in the circle. The answer is returned as an array [x, y].
 * 
 *  
 * Example 1:
 * 
 * Input
 * ["Solution", "randPoint", "randPoint", "randPoint"]
 * [[1.0, 0.0, 0.0], [], [], []]
 * Output
 * [null, [-0.02493, -0.38077], [0.82314, 0.38945], [0.36572, 0.17248]]
 * Explanation
 * Solution solution = new Solution(1.0, 0.0, 0.0);
 * solution.randPoint(); // return [-0.02493, -0.38077]
 * solution.randPoint(); // return [0.82314, 0.38945]
 * solution.randPoint(); // return [0.36572, 0.17248]
 * 
 *  
 * Constraints:
 * 
 * 	0 < radius <= 10^8
 * 	-10^7 <= x_center, y_center <= 10^7
 * 	At most 3 * 10^4 calls will be made to randPoint.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/generate-random-point-in-a-circle/
// discuss: https://leetcode.com/problems/generate-random-point-in-a-circle/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

struct Solution {
        vec![]
    }


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {

    fn new(radius: f64, x_center: f64, y_center: f64) -> Self {
        
    }
    
    fn rand_point(&self) -> Vec<f64> {
        
    }
}

/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(radius, x_center, y_center);
 * let ret_1: Vec<f64> = obj.rand_point();
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_478() {
    }
}
