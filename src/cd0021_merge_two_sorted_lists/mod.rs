//! Merge Two Sorted Lists [leetcode: merge_two_sorted_lists](https://leetcode.com/problems/merge-two-sorted-lists/)
//!
//! Merge two sorted linked lists and return it as a new list. The new list should be made by splicing together the nodes of the first two lists.
//!
//! **Example1:**
//!
//! ```
//! Input: 1->2->4, 1->3->4
//! Output: 1->1->2->3->4->4
//! ```

/// # Solutions
///
/// # Approach 1: Recursion
///
/// * Time complexity: O(n2)
///
/// * Space complexity: O(n)
///
/// * Runtime: 0ms
///
/// * Memory: 2.3MB
///
/// * This is other guy's code not mine
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
///
/// impl Solution {
///     pub fn merge_two_lists(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
///         match (l1, l2) {
///             (Some(node1), None) => Some(node1),
///             (None, Some(node2)) => Some(node2),
///             (Some(mut node1), Some(mut node2)) => {
///                 if node1.val < node2.val {
///                     let n = node1.next.take();
///                     node1.next = Solution::merge_two_lists(n, Some(node2));
///                     Some(node1)
///                 } else {
///                     let n = node2.next.take();
///                     node2.next = Solution::merge_two_lists(Some(node1), n);
///                     Some(node2)
///                 }
///             },
///             _ => None,
///         }
///     }
/// }
/// ```
///
pub fn merge_two_lists(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    match (l1, l2) {
        (Some(node1), None) => Some(node1),
        (None, Some(node2)) => Some(node2),
        (Some(mut node1), Some(mut node2)) => {
            if node1.val < node2.val {
                let n = node1.next.take();
                node1.next = merge_two_lists(n, Some(node2));
                Some(node1)
            } else {
                let n = node2.next.take();
                node2.next = merge_two_lists(Some(node1), n);
                Some(node2)
            }
        },
        _ => None,
    }
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
