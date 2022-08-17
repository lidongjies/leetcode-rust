/**
 * [1302] deepest leaves sum
 * https://leetcode.cn/problems/deepest-leaves-sum/
 *
 * 给你一棵二叉树的根节点 root ，请你返回 层数最深的叶子节点的和 。
 *
 * 示例 1：
 *
 * 输入：root = [1,2,3,4,5,null,6,7,null,null,null,null,8]
 * 输出：15
 * 示例 2：
 *
 * 输入：root = [6,7,8,2,7,1,3,9,null,1,4,null,null,null,5]
 * 输出：19
 *
 *
 * 提示：
 *
 * 树中节点数目在范围 [1, 104] 之间。
 * 1 <= Node.val <= 100
 */
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
use crate::util::tree::{to_tree, TreeNode};
use std::cell::RefCell;

pub struct Solution {}

use std::rc::Rc;
impl Solution {
    pub fn deepest_leaves_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut queue = vec![root];

        while queue.len() > 0 {
            let mut list: Vec<Option<Rc<RefCell<TreeNode>>>> = vec![];
            queue.iter().for_each(|node| {
                let node = node.as_ref().unwrap().borrow();
                if node.left.is_some() {
                    list.push(Some(Rc::clone(node.left.as_ref().unwrap())));
                }
                if node.right.is_some() {
                    list.push(Some(Rc::clone(node.right.as_ref().unwrap())));
                }
            });
            if list.len() == 0 {
                break;
            }
            queue = list;
        }

        let mut sum = 0;
        for node in queue.iter() {
            sum += node.as_ref().unwrap().borrow().val;
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1302() {
        assert_eq!(
            Solution::deepest_leaves_sum(tree![
                1, 2, 3, 4, 5, null, 6, 7, null, null, null, null, 8
            ]),
            15
        );
        assert_eq!(
            Solution::deepest_leaves_sum(tree![
                6, 7, 8, 2, 7, 1, 3, 9, null, 1, 4, null, null, null, 5
            ]),
            19
        );
    }
}
