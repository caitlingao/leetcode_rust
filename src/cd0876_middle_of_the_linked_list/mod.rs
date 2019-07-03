//! Middle of the Linked List [leetcode: middle_of_the_linked_list](https://leetcode.com/problems/middle-of-the-linked-list/)
//!
//! Given a non-empty, singly linked list with head node `head`, return a middle node of linked list.
//!
//! If there are two middle nodes, return the second middle node.
//!
//! **Example1:**
//!
//! ```
//! Input: [1,2,3,4,5]
//! Output: Node 3 from this list (Serialization: [3,4,5])
//! The returned node has value 3.  (The judge's serialization of this node is [3,4,5]).
//! Note that we returned a ListNode object ans, such that:
//! ans.val = 3, ans.next.val = 4, ans.next.next.val = 5, and ans.next.next.next = NULL.
//! ```
//!
//! **Example2:**
//!
//! ```
//! Input: [1,2,3,4,5,6]
//! Output: Node 4 from this list (Serialization: [4,5,6])
//! Since the list has two middle nodes with values 3 and 4, we return the second one.
//! ```
//!
//! **Note**
//!
//! * The number of nodes in the given list will be between `1` and `100`.

/// # Solutions
///
/// # Approach 1: Fast and Slow Pointer
///
/// * Time complexity: O(n)
///
/// * Space complexity: O(1)
///
/// * Runtime: 0ms
///
/// Memory: 2.3MB
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
///     pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
///         let mut fast_p = &head;
///         let mut slow_p = &head;
///
///         while fast_p.is_some() && fast_p.as_ref().unwrap().next.is_some() {
///             slow_p = &slow_p.as_ref().unwrap().next;
///             fast_p = &fast_p.as_ref().unwrap().next.as_ref().unwrap().next;
///         }
///         slow_p.clone()
///     }
/// }
/// ```
///
/// # Approach 2: Output of Array
///
/// * Time complexity: O(n)
///
/// * Space complexity: O(n)
///
/// * Runtime: 0ms
///
/// Memory: 2.5MB
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
///     pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
///         let mut cur = &head;
///         let mut node_vec: Vec<Option<Box<ListNode>>> = vec![];
///
///         while let Some(node) = cur.as_ref() {
///             node_vec.push(Some(node.clone()));
///             cur = &cur.as_ref().unwrap().next;
///         }
///
///         node_vec[node_vec.len() / 2].clone()
///     }
/// }
/// ```
///
pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut fast_p = &head;
    let mut slow_p = &head;

    while fast_p.is_some() && fast_p.as_ref().unwrap().next.is_some() {
        slow_p = &slow_p.as_ref().unwrap().next;
        fast_p = &fast_p.as_ref().unwrap().next.as_ref().unwrap().next;
    }
    slow_p.clone()
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
