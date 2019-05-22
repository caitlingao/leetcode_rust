//! Maximum Depth of Binary Tree[leetcode: maximum_depth_of_binary_tree](https://leetcode.com/problems/maximum-depth-of-binary-tree/)
//!
//! Given a binary tree, find its maximum depth.
//!
//! The maximum depth is the number of nodes along the longest path from the root node down to the farthest leaf node.
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
//! return its depth = 3.

use std::rc::Rc;
use std::cell::RefCell;
use std::cmp::Ord;
/// # Solutions
///
/// # Approach 1: Divde Conquer
///
/// * Time complexity: O(n)
///
/// * Space complexity: O(1)
///
/// * Runtime: 4 ms
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
///     pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
///         match root {
///             Some(node) => {
///                 let left = Self::max_depth(node.borrow().left.clone());
///                 let right = Self::max_depth(node.borrow().right.clone());
///
///                 1 + left.max(right)
///             },
///             _ => 0,
///         }
///     }
/// }
/// ```
///
pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    match root {
        Some(node) => {
            let left = max_depth(node.borrow().left.clone());
            let right = max_depth(node.borrow().right.clone());

            1 + left.max(right)
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
