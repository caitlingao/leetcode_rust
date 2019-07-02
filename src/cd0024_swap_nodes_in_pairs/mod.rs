//! Swap Nodes in Pairs [leetcode: swap_nodes_in_pairs](https://leetcode.com/problems/swap-nodes-in-pairs/)
//!
//! Given a linked list, swap every two adjacent nodes and return its head.
//!
//! You may **not** modify the values in the list's nodes, only nodes itself may be changed.
//!
//! ***Example:***
//!
//! ```
//! Given 1->2->3->4, you should return the list as 2->1->4->3.
//! ```
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
/// Memory: 2.4MB
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
///     pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
///         match head {
///             None => { return None; },
///             Some(mut inner_h) => match inner_h.next {
///                 None => { return Some(inner_h); },
///                 Some(mut inner_s) => {
///                     inner_h.next = Solution::swap_pairs(inner_s.next);
///                     inner_s.next = Some(inner_h);
///                     Some(inner_s)
///                 }
///             }
///         }
///     }
/// }
/// ```
///
pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    match head {
        None => { return None; },
        Some(mut inner_h) => match inner_h.next {
            None => { return Some(inner_h); },
            Some(mut inner_s) => {
                inner_h.next = swap_pairs(inner_s.next);
                inner_s.next = Some(inner_h);
                Some(inner_s)
            }
        }
    }
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
