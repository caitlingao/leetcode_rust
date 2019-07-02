//! Reverse Nodes in k-Group [leetcode: reverse_nodes_in_k_group](https://leetcode.com/problems/reverse-nodes-in-k-group/)
//!
//! Given a linked list, reverse the nodes of a linked list *k* at a time and return its modified list.
//!
//! *k* is a positive integer and is less than or equal to the length of the linked list. If the number of nodes is not a multiple of *k* then left-out nodes in the end should remain as it is.
//!
//! **Example:**
//!
//! ```
//! Given this linked list: 1->2->3->4->5
//!
//! For k = 2, you should return: 2->1->4->3->5
//!
//! For k = 3, you should return: 3->2->1->4->5
//! ```
//!
//! **Note:**
//! * Only constant extra memory is allowed.
//! * You may not alter the values in the list's nodes, only nodes itself may be changed.
//!

/// # Solutions
///
/// # Approach 1: Recursion
///
/// * Time complexity: O(2n)
///
/// * Space complexity: O(n)
///
/// * Runtime: 0ms
///
/// Memory: 2.7MB
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
///     pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
///         let mut head = head;
///         let mut tail = &mut head;
///
///         for _ in 0..k {
///             match tail.as_mut() {
///                 None => return head,
///                 Some(tail_ref) => tail = &mut tail_ref.next,
///             }
///         }
///
///         let tail = tail.take();
///         Solution::add(head, Solution::reverse_k_group(tail, k))
///     }
///
///     pub fn add(head: Option<Box<ListNode>>, tail: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
///         let mut head = head;
///         let mut tail = tail;
///
///         while let Some(mut new_tail) = head.take() {
///             head = new_tail.next.take();
///             new_tail.next = tail.take();
///             tail = Some(new_tail);
///         }
///         tail
///     }
/// }
/// ```
///
pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    let mut head = head;
    let mut tail = &mut head;

    for _ in 0..k {
        match tail.as_mut() {
            None => return head,
            Some(tail_ref) => tail = &mut tail_ref.next,
        }
    }

    let tail = tail.take();
    add(head, reverse_k_group(tail, k))
}

pub fn add(head: Option<Box<ListNode>>, tail: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut head = head;
    let mut tail = tail;

    while let Some(mut new_tail) = head.take() {
        head = new_tail.next.take();
        new_tail.next = tail.take();
        tail = Some(new_tail);
    }
    tail
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
