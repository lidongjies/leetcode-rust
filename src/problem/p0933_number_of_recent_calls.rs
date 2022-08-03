/**
 * [933] Number of Recent Calls
 *
 * You have a RecentCounter class which counts the number of recent requests within a certain time frame.
 * Implement the RecentCounter class:
 * 
 * 	RecentCounter() Initializes the counter with zero recent requests.
 * 	int ping(int t) Adds a new request at time t, where t represents some time in milliseconds, and returns the number of requests that has happened in the past 3000 milliseconds (including the new request). Specifically, return the number of requests that have happened in the inclusive range [t - 3000, t].
 * 
 * It is guaranteed that every call to ping uses a strictly larger value of t than the previous call.
 *  
 * Example 1:
 * 
 * Input
 * ["RecentCounter", "ping", "ping", "ping", "ping"]
 * [[], [1], [100], [3001], [3002]]
 * Output
 * [null, 1, 2, 3, 3]
 * Explanation
 * RecentCounter recentCounter = new RecentCounter();
 * recentCounter.ping(1);     // requests = [<u>1</u>], range is [-2999,1], return 1
 * recentCounter.ping(100);   // requests = [<u>1</u>, <u>100</u>], range is [-2900,100], return 2
 * recentCounter.ping(3001);  // requests = [<u>1</u>, <u>100</u>, <u>3001</u>], range is [1,3001], return 3
 * recentCounter.ping(3002);  // requests = [1, <u>100</u>, <u>3001</u>, <u>3002</u>], range is [2,3002], return 3
 * 
 *  
 * Constraints:
 * 
 * 	1 <= t <= 10^9
 * 	Each test case will call ping with strictly increasing values of t.
 * 	At most 10^4 calls will be made to ping.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/number-of-recent-calls/
// discuss: https://leetcode.com/problems/number-of-recent-calls/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

struct RecentCounter {
        vec![]
    }


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RecentCounter {

    fn new() -> Self {
        
    }
    
    fn ping(&self, t: i32) -> i32 {
        
    }
}

/**
 * Your RecentCounter object will be instantiated and called as such:
 * let obj = RecentCounter::new();
 * let ret_1: i32 = obj.ping(t);
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_933() {
    }
}