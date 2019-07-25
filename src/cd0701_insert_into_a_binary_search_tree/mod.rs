//! Insert into a Binary Search Tree[leetcode: insert_into_a_binary_search_tree](https://leetcode.com/problems/insert-into-a-binary-search-tree/)
//!
//! Given the root node of a binary search tree (BST) and a value to be inserted into the tree, insert the value into the BST. Return the root node of the BST after the insertion. It is guaranteed that the new value does not exist in the original BST.
//!
//! Note that there may exist multiple valid ways for the insertion, as long as the tree remains a BST after insertion. You can return any of them.
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
//! And the value to insert: 5
//! ```
//! You can return this binary search tree:
//! ```
//!          4
//!        /   \
//!       2     7
//!      / \   /
//!     1   3 5
//! ```
//! This tree is also valid:
//! ```
//!         5
//!       /   \
//!      2     7
//!     / \
//!    1   3
//!         \
//!          4
//! ```

/// # Solutions
///
/// # Approach 1: BFS
///
/// * Time complexity: O(n)
///
/// * Space complexity: O(1)
///
/// * Runtime: 12 ms
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
///     pub fn insert_into_bst(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
///         Self::insert(&root, val);
///         root
///     }
///     fn insert(node: &Option<Rc<RefCell<TreeNode>>>, val: i32) {
///         if let Some(n) = node {
///             let mut n = n.borrow_mut();
///             let target = if val > n.val { &mut n.right } else { &mut n.left };
///
///             if target.is_some() {
///                 return Self::insert(target, val);
///             }
///
///              *target = Some(Rc::new(RefCell::new(TreeNode::new(val))));
///         }
///     }
/// }
/// ```
///
///
pub fn insert_into_bst(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
    insert(&root, val);
    root
}
fn insert(node: &Option<Rc<RefCell<TreeNode>>>, val: i32) {
    if let Some(n) = node {
        let mut n = n.borrow_mut();
        let target = if val > n.val { &mut n.right } else { &mut n.left };

        if target.is_some() {
            return insert(target, val);
        }

         *target = Some(Rc::new(RefCell::new(TreeNode::new(val))));
    }
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
