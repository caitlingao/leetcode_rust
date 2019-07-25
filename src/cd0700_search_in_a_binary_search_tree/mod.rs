//! Search in a Binary Search Tree[leetcode: search_in_a_binary_search_tree](https://leetcode.com/problems/search-in-a-binary-search-tree/)
//!
//! Given the root node of a binary search tree (BST) and a value. You need to find the node in the BST that the node's value equals the given value. Return the subtree rooted with that node. If such node doesn't exist, you should return NULL.
//!
//! For example:
//!
//! ```
//! Given the tree:
//!         4
//!        / \
//!       2   7
//!      / \
//!     1   3
//!
//! And the value to search: 2
//! ```
//! You should return this subtree:
//! ```
//!       2
//!      / \
//!     1   3
//! ```
//! In the example above, if we want to search the value `5`, since there is no node with value `5`, we should return `NULL`.
//!
//! Note that an empty tree is represented by `NULL`, therefore you would see the expected output (serialized tree format) as `[]`, not `null`.

/// # Solutions
///
/// # Approach 1: BFS
///
/// * Time complexity: O(n)
///
/// * Space complexity: O(1)
///
/// * Runtime: 4 ms
///
/// * Memory: 3.4 MB
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
///     pub fn search_bst(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
///         let mut r = root.clone();
///         while let Some(node) = r {
///             if node.borrow().val == val { return Some(node); }
///             if node.borrow().val > val {
///                 r = node.borrow().left.clone();
///             } else {
///                 r = node.borrow().right.clone();
///             }
///         }
///        None
///     }
/// }
/// ```
///
///
pub fn search_bst(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
    let mut r = root.clone();
    while let Some(node) = r {
        if node.borrow().val == val { return Some(node); }
        if node.borrow().val > val {
            r = node.borrow().left.clone();
        } else {
            r = node.borrow().right.clone();
        }
    }
   None
}

use std::rc::Rc;
use std::cell::RefCell;
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
