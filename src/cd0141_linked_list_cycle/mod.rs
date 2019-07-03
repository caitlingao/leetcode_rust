//! Linked List Cycle [leetcode: linked_list_cycle](https://leetcode.com/problems/linked-list-cycle/)
//!
//! Given a linked list, determine if it has a cycle in it.
//!
//! To represent a cycle in the given linked list, we use an integer `pos` which represents the position (0-indexed) in the linked list where tail connects to. If `pos` is `-1`, then there is no cycle in the linked list.
//!
//! **Example1:**
//!
//! ```
//! Input: head = [3,2,0,-4], pos = 1
//! Output: true
//! Explanation: There is a cycle in the linked list, where tail connects to the second node.
//! ```
//! <div>
//! <img alt="" src="https://assets.leetcode.com/uploads/2018/12/07/circularlinkedlist.png" style="width: 300px; height: 97px; margin-top: 8px; margin-bottom: 8px;">
//! </div>
//!
//! **Example2:**
//!
//! ```
//! Input: head = [1,2], pos = 0
//! Output: true
//! Explanation: There is a cycle in the linked list, where tail connects to the first node.
//! ```
//!
//! <div>
//! <img alt="" src="https://assets.leetcode.com/uploads/2018/12/07/circularlinkedlist_test2.png" style="width: 141px; height: 74px;">
//! </div>
//!
//! **Example3:**
//!
//! ```
//! Input: head = [1], pos = -1
//! Output: false
//! Explanation: There is no cycle in the linked list.
//! ```
//!
//! <div>
//! <img alt="" src="https://assets.leetcode.com/uploads/2018/12/07/circularlinkedlist_test3.png" style="width: 45px; height: 45px;">
//! </div>
//!
//! **Follow up:**
//!
//! Can you solve it using *O(1)* (i.e. constant) memory?

use std::collections::HashSet;
/// # Solutions
///
/// # Approach 1: Hash Table
///
/// * Time complexity: O(n)
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
/// // #[derive(Hash, Eq, PartialEq, Debug, Clone)]
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
/// use std::collections::HashSet;
/// impl Solution {
///     pub fn has_cycle(mut head: Option<Box<ListNode>>) -> bool {
///         let mut vikings = HashSet::new();
///         let mut cur = &mut head;
///         while let Some(cur_mut) = cur.as_mut() {
///             let cur_mut = cur.as_mut().unwrap();
///             if vikings.contains(cur_mut) {
///                 return true;
///             } else {
///                 vikings.insert(cur_mut.clone());
///             }
///             cur = &mut cur_mut.next;
///         }
///         false
///     }
/// }
/// ```
///
/// # Approach 2: Fast and Slow Pointer
///
/// * Time complexity: O(n)
///
/// * Space complexity: O(1)
///
/// * Runtime: 0ms
///
/// Memory: 2.4MB
///
/// ```rust
///
/// // Definition for singly-linked list.
/// // #[derive(Hash, Eq, PartialEq, Debug, Clone)]
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
///     pub fn has_cycle(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
///         let mut fast_p = &head;
///         let mut slow_p = &head;
///
///         while fast_p.is_some() && fast_p.as_ref().unwrap().next.is_some() {
///             slow_p = &slow_p.as_ref().unwrap().next;
///             fast_p = &fast_p.as_ref().unwrap().next.as_ref().unwrap().next;
///
///             if slow_p == fast_p { return true; }
///         }
///         false
///     }
/// }
/// ```
///
pub fn has_cycle(mut head: Option<Box<ListNode>>) -> bool {
    let mut vikings = HashSet::new();
    let mut cur = &mut head;
    while let Some(_cur_mut) = cur.as_mut() {
        let cur_mut = cur.as_mut().unwrap();
        if vikings.contains(cur_mut) {
            return true;
        } else {
            vikings.insert(cur_mut.clone());
        }
        cur = &mut cur_mut.next;
    }
    false
}

// Definition for singly-linked list.
#[derive(Hash, Eq, PartialEq, Debug, Clone)]
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
