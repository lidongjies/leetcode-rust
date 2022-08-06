/**
 * [2013] Detect Squares
 *
 * You are given a stream of points on the X-Y plane. Design an algorithm that:
 * 
 * 	Adds new points from the stream into a data structure. Duplicate points are allowed and should be treated as different points.
 * 	Given a query point, counts the number of ways to choose three points from the data structure such that the three points and the query point form an axis-aligned square with positive area.
 * 
 * An axis-aligned square is a square whose edges are all the same length and are either parallel or perpendicular to the x-axis and y-axis.
 * Implement the DetectSquares class:
 * 
 * 	DetectSquares() Initializes the object with an empty data structure.
 * 	void add(int[] point) Adds a new point point = [x, y] to the data structure.
 * 	int count(int[] point) Counts the number of ways to form axis-aligned squares with point point = [x, y] as described above.
 * 
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/09/01/image.png" style="width: 869px; height: 504px;" />
 * Input
 * ["DetectSquares", "add", "add", "add", "count", "count", "add", "count"]
 * [[], [[3, 10]], [[11, 2]], [[3, 2]], [[11, 10]], [[14, 8]], [[11, 2]], [[11, 10]]]
 * Output
 * [null, null, null, null, 1, 0, null, 2]
 * Explanation
 * DetectSquares detectSquares = new DetectSquares();
 * detectSquares.add([3, 10]);
 * detectSquares.add([11, 2]);
 * detectSquares.add([3, 2]);
 * detectSquares.count([11, 10]); // return 1. You can choose:
 *                                //   - The first, second, and third points
 * detectSquares.count([14, 8]);  // return 0. The query point cannot form a square with any points in the data structure.
 * detectSquares.add([11, 2]);    // Adding duplicate points is allowed.
 * detectSquares.count([11, 10]); // return 2. You can choose:
 *                                //   - The first, second, and third points
 *                                //   - The first, third, and fourth points
 * 
 *  
 * Constraints:
 * 
 * 	point.length == 2
 * 	0 <= x, y <= 1000
 * 	At most 3000 calls in total will be made to add and count.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/detect-squares/
// discuss: https://leetcode.com/problems/detect-squares/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

struct DetectSquares {
        false
    }


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl DetectSquares {

    fn new() -> Self {
        
    }
    
    fn add(&self, point: Vec<i32>) {
        
    }
    
    fn count(&self, point: Vec<i32>) -> i32 {
        
    }
}

/**
 * Your DetectSquares object will be instantiated and called as such:
 * let obj = DetectSquares::new();
 * obj.add(point);
 * let ret_2: i32 = obj.count(point);
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2013() {
    }
}