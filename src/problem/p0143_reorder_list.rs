/**
 * [143] Reorder List
 *
 * You are given the head of a singly linked-list. The list can be represented as:
 * 
 * L0 &rarr; L1 &rarr; &hellip; &rarr; Ln - 1 &rarr; Ln
 * 
 * Reorder the list to be on the following form:
 * 
 * L0 &rarr; Ln &rarr; L1 &rarr; Ln - 1 &rarr; L2 &rarr; Ln - 2 &rarr; &hellip;
 * 
 * You may not modify the values in the list's nodes. Only nodes themselves may be changed.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/03/04/reorder1linked-list.jpg" style="width: 422px; height: 222px;" />
 * Input: head = [1,2,3,4]
 * Output: [1,4,2,3]
 * 
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/03/09/reorder2-linked-list.jpg" style="width: 542px; height: 222px;" />
 * Input: head = [1,2,3,4,5]
 * Output: [1,5,2,4,3]
 * 
 *  
 * Constraints:
 * 
 * 	The number of nodes in the list is in the range [1, 5 * 10^4].
 * 	1 <= Node.val <= 1000
 * 
 */
pub struct Solution {}
use crate::util::linked_list::{ListNode, to_list};

// problem: https://leetcode.com/problems/reorder-list/
// discuss: https://leetcode.com/problems/reorder-list/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_143() {
    }
}
