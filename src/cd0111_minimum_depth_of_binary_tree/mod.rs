//! Minimum Depth of Binary Tree[leetcode: minimum_depth_of_binary_tree](https://leetcode.com/problems/minimum-depth-of-binary-tree/)
//!
//! Given a binary tree, find its minimum depth.
//!
//!The minimum depth is the number of nodes along the shortest path from the root node down to the nearest leaf node.
//!
//! **Note:** A leaf is a node with no children.
//!
//! **example:**
//! Given binary tree `[3,9,20,null,null,15,7]`,
//!
//! ```
//!     3
//!    / \
//!   9  20
//!     /  \
//!    15   7
//! ```
//! return its depth = 2.

use std::rc::Rc;
use std::cell::RefCell;
use std::cmp::Ord;
/// # Solutions
///
/// # Approach 1: DFS
///
/// * Time complexity: O(n)
///
/// * Space complexity: O(1)
///
/// * Runtime: 0 ms
///
/// * Memory: 3.1 MB
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
/// use std::cmp::Ord;
/// impl Solution {
///     pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
///         match root {
///             Some(node) => {
///                 let left = Self::min_depth(node.borrow().left.clone());
///                 let right = Self::min_depth(node.borrow().right.clone());
///
///                 if left == 0 || right == 0 {
///                     1 + left + right
///                 } else {
///                     1 + left.min(right)
///                 }
///             },
///             _ => 0,
///         }
///     }
/// }
/// ```
///
/// # Approach 2: BFS
///
/// * Time complexity: O(n)
///
/// * Space complexity: O(1)
///
/// * Runtime: 0 ms
///
/// * Memory: 3.2 MB
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
///     pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
///         if root.is_none() { return 0; }
///
///         let mut deque: VecDeque<Option<Rc<RefCell<TreeNode>>>> = VecDeque::new();
///         let mut depth = 0;
///         deque.push_back(root);
///
///         while !deque.is_empty() {
///             depth += 1;
///             let mut added = false;
///             let level_size = deque.len();
///
///             for _i in 0..level_size {
///                 let n = deque.pop_front();
///                 if let(Some(Some(node))) = n {
///                     added = true;
///                     if node.borrow().left.is_none() && node.borrow().right.is_none() { return depth; }
///                     if node.borrow().left.is_some() { deque.push_back(node.borrow().left.clone()); }
///                     if node.borrow().right.is_some() { deque.push_back(node.borrow().right.clone());}
///                 }
///             }
///             if !added { break; }
///         }
///         depth
///     }
/// }
/// ```
///
pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    match root {
        Some(node) => {
            let left = min_depth(node.borrow().left.clone());
            let right = min_depth(node.borrow().right.clone());

            if left == 0 || right == 0 {
                1 + left + right
            } else {
                1 + left.min(right)
            }
        },
        _ => 0,
    }
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
