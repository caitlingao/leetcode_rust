//! Binary Tree Preorder Traversal[leetcode: binary_tree_preorder_traversal](https://leetcode.com/problems/binary-tree-preorder-traversal/)
//!
//! Given a binary tree, return the *preorder* traversal of its nodes' values.
//!
//! **Example:**
//!
//! ```
//! Input: [1,null,2,3]
//!    1
//!     \
//!      2
//!     /
//!    3
//!
//! Output: [1,2,3]
//! ```
//! **Follow up:** Recursive solution is trivial, could you do it iteratively?

/// # Solutions
///
/// # Approach 1: Recursive Approach
///
/// * Time complexity: O(n)
///
/// * Space complexity: O(logn)
///
/// * Runtime: 0 ms
///
/// * Memory: 2.4 MB
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
///     pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
///         let mut result: Vec<i32> = vec![];
///         if root.is_none() { return result; }
///
///         Self::_post_order(root, &mut result);
///         result
///     }
///     fn _post_order(root: Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
///         match root {
///             Some(node) => {
///                 result.push(node.borrow().val);
///                 Self::_post_order(node.borrow().left.clone(), result);
///                 Self::_post_order(node.borrow().right.clone(), result);
///             },
///             None => { return; }
///         }
///     }
/// }
/// ```
///
/// # Approach 2: Iterating method using Stack
///
/// * Time complexity: O(n)
///
/// * Space complexity: O(n)
///
/// * Runtime: 0 ms
///
/// * Memory: 2.4 MB
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
///     pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
///         let mut result = vec![];
///         if root.is_none() { return result; }
///
///         let mut stack: Vec<Rc<RefCell<TreeNode>>> = Vec::new();
///         let mut r = root.clone();
///         while r.is_some() || !stack.is_empty() {
///             while let Some(node) = r {
///                 result.push(node.borrow().val);
///                 stack.push(node.clone());
///                 r = node.borrow().left.clone();
///             }
///             r = stack.pop();
///             if let Some(node) = r {
///                 r = node.borrow().right.clone();
///             }
///         }
///         result
///
///     }
/// }
/// ```
///
pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut result = vec![];
    if root.is_none() { return result; }

    let mut stack: Vec<Rc<RefCell<TreeNode>>> = Vec::new();
    let mut r = root.clone();
    while r.is_some() || !stack.is_empty() {
        while let Some(node) = r {
            result.push(node.borrow().val);
            stack.push(node.clone());
            r = node.borrow().left.clone();
        }
        r = stack.pop();
        if let Some(node) = r {
            r = node.borrow().right.clone();
        }
    }
    result
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
