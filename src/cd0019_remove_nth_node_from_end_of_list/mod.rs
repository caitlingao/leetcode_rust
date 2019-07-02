//! Remove Nth Node From End of List [leetcode: remove_nth_node_from_end_of_list](https://leetcode.com/problems/remove-nth-node-from-end-of-list/)
//!
//! Given a linked list, remove the *n*-th node from the end of list and return its head.
//!
//! **Example:**
//!
//! ```
//! Given linked list: 1->2->3->4->5, and n = 2.
//!
//! After removing the second node from the end, the linked list becomes 1->2->3->5.
//! ```
//!
//! **Note:**
//!
//! Given *n* will always be valid.
//!
//! **Follow up:**
//!
//! Could you do this in one pass?

/// # Solutions
///
/// # Approach 1: Two pass algorithm
///
/// * Time complexity: O(L)
///
/// * Space complexity: O(1)
///
/// ```rust
///
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
///     pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
///         let mut dummy = Some(Box::new(ListNode { val: 0, next: head }));
///         let mut cur = &mut dummy;
///         let mut length = 0;
///
///         while let Some(_node) = cur.as_mut() {
///             cur = &mut cur.as_mut().unwrap().next;
///             if let Some(_inner_node) = cur { length += 1; }
///         }
///
///         let mut new_cur = dummy.as_mut();
///         let idx = length - n;
///
///         for _ in 0..idx {
///             new_cur = new_cur.unwrap().next.as_mut();
///         }
///
///         let next = new_cur.as_mut().unwrap().next.as_mut().unwrap().next.take();
///         new_cur.as_mut().unwrap().next = next;
///
///         dummy.unwrap().next
///     }
/// }
/// ```
///
pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    let mut dummy = Some(Box::new(ListNode { val: 0, next: head }));
    let mut cur = &mut dummy;
    let mut length = 0;

    while let Some(_node) = cur.as_mut() {
        cur = &mut cur.as_mut().unwrap().next;
        if let Some(_inner_node) = cur { length += 1; }
    }

    let mut new_cur = dummy.as_mut();
    let idx = length - n;

    for _ in 0..idx {
        new_cur = new_cur.unwrap().next.as_mut();
    }

    let next = new_cur.as_mut().unwrap().next.as_mut().unwrap().next.take();
    new_cur.as_mut().unwrap().next = next;

    dummy.unwrap().next
}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

#[allow(dead_code)]
impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}
