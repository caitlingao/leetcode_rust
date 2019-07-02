//! Reverse Linked List [leetcode: reverse_linked_list](https://leetcode.com/problems/reverse-linked-list)
//!
//! Reverse a singly linked list.
//!
//! ***Example:***
//!
//! ```
//! Input: 1->2->3->4->5->NULL
//!
//! Output: 5->4->3->2->1->NULL
//! ```
//!
//! ***Follow up:***
//!
//! A linked list can be reversed either iteratively or recursively. Could you implement both?

/// # Solutions
///
/// # Approach 1: Iterative
///
/// * Time complexity: O(n)
///
/// * Space complexity: O(1)
///
/// ```rust
///
/// // Definition for singly-linked list.
/// // #[derive(PartialEq, Eq, Debug)]
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
///
/// impl Solution {
///     pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
/// 	    if head.is_none() { return None; }
///
/// 	    let mut prev = None;
/// 	    let mut current = head;
/// 	    while let Some(mut tmp) = current.take() {
/// 	        let next = tmp.next.take();
/// 	        tmp.next = prev.take();
/// 	        prev = Some(tmp);
/// 	        current = next;
/// 	    }
///
/// 	    prev
///     }
/// }
/// ```
///
pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if head.is_none() { return None; }

    let mut prev = None;
    let mut current = head;
    while let Some(mut tmp) = current.take() {
	let next = tmp.next.take();
	tmp.next = prev.take();
	prev = Some(tmp);
	current = next;
    }

    prev
}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Debug)]
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
