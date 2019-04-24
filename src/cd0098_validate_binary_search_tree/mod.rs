//! Validate Binary Search Tree[leetcode: validate_binary_search_tree](https://leetcode.com/problems/validate-binary-search-tree/)
//!
//! Given a binary tree, determine if it is a valid binary search tree (BST).
//!
//! Assume a BST is defined as follows:
//!
//! * The left subtree of a node contains only nodes with keys *less than* the node's key.
//! * The right subtree of a node contains only nodes with keys *greater than* the node's key.
//! * Both the left and right subtrees must also be binary search trees.
//!
//! ***Example1:***
//!
//! ```
//! Input:
//!     2
//!    / \
//!   1   3
//! Output: true
//! ```
//!
//! ***Example2:***
//!
//! ```
//!     5
//!    / \
//!   1   4
//!      / \
//!     3   6
//! Output: false
//! Explanation: The input is: [5,1,4,null,null,3,6]. The root node's value
//!              is 5 but its right child's value is 4.
//! ```

use std::rc::Rc;
use std::cell::RefCell;
/// # Solutions
///
/// # Approach 1: Recursion
///
/// * Time complexity: O(n)
///
/// * Space complexity: O(n)
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
///     pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
///         fn valid(root: &Option<Rc<RefCell<TreeNode>>>, min_limit: Option<i32>, max_limit: Option<i32>) -> bool {
///             match root {
///                 Some(node) => {
///                     let node = node.borrow();
///
///                     if let Some(v) = min_limit {
///                         if node.val <= v { return false; }
///                     }
///                     if let Some(v) = max_limit {
///                         if node.val >= v { return false; }
///                     }
///                     return valid(&node.left, min_limit, Some(node.val)) && valid(&node.right, Some(node.val), max_limit);
///                 },
///                 _ => { true }
///             }
///             // if let Some(n) = node {
///             //     let n = n.borrow();
///             //
///             //     if let Some(v) = min_limit {
///             //         if n.val <= v { return false; }
///             //     }
///             //     if let Some(v) = max_limit {
///             //         if n.val >= v { return false; }
///             //     }
///             //     return valid(&n.left, min_limit, Some(n.val)) && valid(&n.right, Some(n.val), max_limit);
///             // } else {
///             //     true
///             // }
///
///         }
///
///         valid(&root, None, None)
///     }
/// }
/// ```
///
/// # Approach 2: inorder
///
/// * Time complexity: O(n)
///
/// * Space complexity: O(n)
///
/// * Runtime: 4 ms
///
/// * Memory: 3.8 MB
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
///     pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
///         fn inorder(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
///             let mut result = vec![];
///
///             if let Some(node) = root {
///                 let mut left = inorder(&node.borrow().left);
///                 let mut right = inorder(&node.borrow().right);
///
///                 result.append(&mut left);
///                 result.push(node.borrow().val);
///                 result.append(&mut right);
///             }
///
///             result
///         }
///
///        let result = inorder(&root);
///        for i in 1..result.len() {
///            if result[i] <= result[i-1] { return false; }
///        }
///
///        true
///     }
/// }
///
pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    fn valid(node: &Option<Rc<RefCell<TreeNode>>>, min_limit: Option<i32>, max_limit: Option<i32>) -> bool {
        match node {
            Some(n) => {
                let n = n.borrow();

                if let Some(v) = min_limit {
                    if n.val <= v { return false; }
                }
                if let Some(v) = max_limit {
                    if n.val >= v { return false; }
                }
                return valid(&n.left, min_limit, Some(n.val)) && valid(&n.right, Some(n.val), max_limit);
            },
            _ => { true }
        }

    }

    valid(&root, None, None)
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
