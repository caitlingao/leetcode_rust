//! Binary Tree Level Order Traversal[leetcode: binary_tree_level_order_traversal](https://leetcode.com/problems/binary-tree-level-order-traversal/)
//!
//! Given a binary tree, return the level order traversal of its nodes' values. (ie, from left to right, level by level).
//!
//! For example:
//! Given binary tree `[3,9,20,null,null,15,7]`,
//!
//! ```
//!     3
//!    / \
//!   9  20
//!     /  \
//!    15   7
//! ```
//! return its level order traversal as:
//! ```
//! [
//!   [3],
//!   [9,20],
//!   [15,7]
//! ]
//! ```

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;
/// # Solutions
///
/// # Approach 1: BFS
///
/// * Time complexity: O(n)
///
/// * Space complexity: O(n)
///
/// * Runtime: 0 ms
///
/// * Memory: 2.6 MB
///
/// ```rust
/// // Definition for a binary tree node.
/// // #[derive(Debug, PartialEq, Eq)]
/// // pub struct TreeNode {
/// //   pub val: i32,
/// //   pub left: Option<Rc<RefCell<TreeNode>>>,
/// //   pub right: Option<Rc<RefCell<TreeNode>>>,
/// // }
/// //
/// // impl TreeNode {
/// //   #[inline]
/// //   pub fn new(val: i32) -> Self {
/// //     TreeNode {
/// //       val,
/// //       left: None,
/// //       right: None
/// //     }
/// //   }
/// // }
/// use std::rc::Rc;
/// use std::cell::RefCell;
/// use std::collections::VecDeque;
/// impl Solution {
///     pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
///         let mut result: Vec<Vec<i32>> = vec![];
///         if root.is_none() { return result; }
///
///         let mut deque: VecDeque<Option<Rc<RefCell<TreeNode>>>> = VecDeque::new();
///         deque.push_back(root);
///
///         while !deque.is_empty() {
///             let mut current_level = vec![];
///             let mut added = false;
///             let level_size = deque.len();
///
///             for _i in 0..level_size {
///                 let n = deque.pop_front();
///                 if let Some(Some(node)) = n {
///                     current_level.push(node.borrow().val);
///                     added = true;
///                     if !node.borrow().left.is_none() { deque.push_back(node.borrow().left.clone()); }
///                     if !node.borrow().right.is_none() { deque.push_back(node.borrow().right.clone()); }
///                 }
///             }
///
///             if !added { break; }
///
///             result.push(current_level);
///         }
///
///         result
///     }
/// }
/// ```
///
/// # Approach 2: DFS
///
/// * Time complexity: O(n)
///
/// * Space complexity: O(n)
///
/// * Runtime: 0 ms
///
/// * Memory: 2.7 MB
///
/// ```rust
/// // Definition for a binary tree node.
/// // #[derive(Debug, PartialEq, Eq)]
/// // pub struct TreeNode {
/// //   pub val: i32,
/// //   pub left: Option<Rc<RefCell<TreeNode>>>,
/// //   pub right: Option<Rc<RefCell<TreeNode>>>,
/// // }
/// //
/// // impl TreeNode {
/// //   #[inline]
/// //   pub fn new(val: i32) -> Self {
/// //     TreeNode {
/// //       val,
/// //       left: None,
/// //       right: None
/// //     }
/// //   }
/// // }
/// use std::rc::Rc;
/// use std::cell::RefCell;
/// impl Solution {
///     pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
///         let mut result: Vec<Vec<i32>> = vec![];
///         Self::_dfs(&mut result, root, 0);
///         result
///     }
///
///     pub fn _dfs(result: &mut Vec<Vec<i32>>, root: Option<Rc<RefCell<TreeNode>>>, level: usize) {
///         if let Some(node) = root {
///             if result.len() == level {
///                 result.push(vec![node.borrow().val]);
///             } else {
///                 result[level].push(node.borrow().val);
///             }
///             Self::_dfs(result, node.borrow().left.clone(), level + 1);
///             Self::_dfs(result, node.borrow().right.clone(), level + 1)
///         }
///     }
/// }
/// ```
///
pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = vec![];
    if root.is_none() { return result; }

    let mut deque: VecDeque<Option<Rc<RefCell<TreeNode>>>> = VecDeque::new();
    deque.push_back(root);

    while !deque.is_empty() {
        let mut current_level = vec![];
        let mut added = false;
        let level_size = deque.len();

        for _i in 0..level_size {
            let n = deque.pop_front();
            if let Some(Some(node)) = n {
                current_level.push(node.borrow().val);
                added = true;
                if !node.borrow().left.is_none() { deque.push_back(node.borrow().left.clone()); }
                if !node.borrow().right.is_none() { deque.push_back(node.borrow().right.clone()); }
            }
        }

        if !added { break; }

        result.push(current_level);
    }

    result
}
// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}
