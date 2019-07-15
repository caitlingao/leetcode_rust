//! Add Two Numbers [leetcode: add_two_numbers](https://leetcode.com/problems/add-two-numbers/)
//!
//! You are given two **non-empty** linked lists representing two non-negative integers. The digits are stored in **reverse order** and each of their nodes contain a single digit. Add the two numbers and return it as a linked list.
//!
//! You may assume the two numbers do not contain any leading zero, except the number 0 itself.
//!
//! **Example:**
//!
//! ```
//! Input: (2 -> 4 -> 3) + (5 -> 6 -> 4)
//! Output: 7 -> 0 -> 8
//! Explanation: 342 + 465 = 807.
//! ```
//!
/// # Solutions
///
/// # Approach 1: Elementary Math
///
/// * Time complexity: O(max(m, n))
///
/// * Space complexity: O(max(m, n))
///
/// * Runtime: 4 ms
/// * Memory: 2.3 MB
///
/// ```rust
/// // Definition for singly-linked list.
/// // #[derive(PartialEq, Eq, Clone, Debug)]
/// // pub struct ListNode {
/// //   pub val: i32,
/// //   pub next: Option<Box<ListNode>>
/// // }
/// //
/// // impl ListNode {
/// //   #[inline]
/// //   fn new(val: i32) -> Self {
/// //     ListNode {
/// //       next: None,
/// //       val
/// //     }
/// //   }
/// // }
/// impl Solution {
///     pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
///         let mut dummy1 = l1;
///         let mut dummy2 = l2;
///         let mut root = Some(Box::new(ListNode::new(0)));
///         let mut curr = &mut root;
///         let mut carry = 0;
///
///         while dummy1.is_some() || dummy2.is_some() {
///             match curr {
///                 Some(inner_node) => {
///                     let first = dummy1.take().unwrap_or(Box::new(ListNode::new(0)));
///                     let second = dummy2.take().unwrap_or(Box::new(ListNode::new(0)));
///                     let mut sum = first.val + second.val + carry;
///                     carry = sum / 10;
///                     sum = sum % 10;
///                     inner_node.next.get_or_insert(Box::new(ListNode::new(sum)));
///                     curr = &mut inner_node.next;
///                     dummy1 = first.next;
///                     dummy2 = second.next;
///                 },
///                 None => break,
///             }
///         }
///
///         if carry == 1 {
///             if let Some(node) = curr {
///                 node.next.get_or_insert(Box::new(ListNode::new(1)));
///             }
///         }
///
///         root.unwrap().next
///     }
/// }
/// ```
///
pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut dummy1 = l1;
    let mut dummy2 = l2;
    let mut root = Some(Box::new(ListNode::new(0)));
    let mut curr = &mut root;
    let mut carry = 0;

    while dummy1.is_some() || dummy2.is_some() {
        match curr {
            Some(inner_node) => {
                let first = dummy1.take().unwrap_or(Box::new(ListNode::new(0)));
                let second = dummy2.take().unwrap_or(Box::new(ListNode::new(0)));
                let mut sum = first.val + second.val + carry;
                carry = sum / 10;
                sum = sum % 10;
                inner_node.next.get_or_insert(Box::new(ListNode::new(sum)));
                curr = &mut inner_node.next;
                dummy1 = first.next;
                dummy2 = second.next;
            },
            None => break,
        }
    }

    if carry == 1 {
        if let Some(node) = curr {
            node.next.get_or_insert(Box::new(ListNode::new(1)));
        }
    }

    root.unwrap().next
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}
