/**
 * [501] Find Mode in Binary Search Tree
 *
 * Given the root of a binary search tree (BST) with duplicates, return all the <a href="https://en.wikipedia.org/wiki/Mode_(statistics)" target="_blank">mode(s)</a> (i.e., the most frequently occurred element) in it.
 * If the tree has more than one mode, return them in any order.
 * Assume a BST is defined as follows:
 * 
 * 	The left subtree of a node contains only nodes with keys less than or equal to the node's key.
 * 	The right subtree of a node contains only nodes with keys greater than or equal to the node's key.
 * 	Both the left and right subtrees must also be binary search trees.
 * 
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/03/11/mode-tree.jpg" style="width: 142px; height: 222px;" />
 * Input: root = [1,null,2,2]
 * Output: [2]
 * 
 * Example 2:
 * 
 * Input: root = [0]
 * Output: [0]
 * 
 *  
 * Constraints:
 * 
 * 	The number of nodes in the tree is in the range [1, 10^4].
 * 	-10^5 <= Node.val <= 10^5
 * 
 *  
 * Follow up: Could you do that without using any extra space? (Assume that the implicit stack space incurred due to recursion does not count).
 */
pub struct Solution {}
use crate::util::tree::{TreeNode, to_tree};

// problem: https://leetcode.com/problems/find-mode-in-binary-search-tree/
// discuss: https://leetcode.com/problems/find-mode-in-binary-search-tree/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn find_mode(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        vec![]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_501() {
    }
}
