//! Kth Largest Element in a Stream [leetcode: kth_largest_element_in_a_stream](https://leetcode.com/problems/kth-largest-element-in-a-stream/)
//!
//! Design a class to find the **k**th largest element in a stream. Note that it is the kth largest element in the sorted order, not the kth distinct element.
//!
//! Your `KthLargest` class will have a constructor which accepts an integer `k` and an integer array `nums`, which contains initial elements from the stream. For each call to the method `KthLargest.add`, return the element representing the kth largest element in the stream.
//!
//! ***Example:***
//!
//! ```
//! int k = 3;
//! int[] arr = [4,5,8,2];
//! KthLargest kthLargest = new KthLargest(3, arr);
//! kthLargest.add(3);   // returns 4
//! kthLargest.add(5);   // returns 5
//! kthLargest.add(10);  // returns 5
//! kthLargest.add(9);   // returns 8
//! kthLargest.add(4);   // returns 8
//! ```
//!
//! ***Notes***
//!
//! You may assume that `nums`' length ≥ `k-1` and `k` ≥ 1.

/// # Solutions
///
/// # Approach 1: BinaryHeap
///
/// * Time complexity: O(log2 k)
///
/// * Space complexity: O(1)
///
/// ```rust
/// use std::collections::BinaryHeap;
///
/// struct KthLargest {
///     k: usize,
///     heap: BinaryHeap<i32>,
/// }
///
/// impl KthLargest {
///     fn new(k: i32, nums: Vec<i32>) -> Self {
///         let mut kth = KthLargest { k: k as usize, heap: BinaryHeap::new(), };
///         for n in nums {
///             kth.add(n);
///         }
///         kth
///     }
///
///     fn add(&mut self, val: i32) -> i32 {
///         if self.heap.len() < self.k || *self.heap.peek().unwrap() >= -val {
///             self.heap.push(-val);
///             if self.heap.len() > self.k {
///                 self.heap.pop();
///             }
///         }
///         -*self.heap.peek().unwrap()
///     }
/// }
/// ```
///
/// # Approach 2: Vec
///
/// * Time complexity: O(n log n)
///
/// * Space complexity: O(1)
///
/// ```rust
/// struct KthLargest {
///     k: usize,
///     v: Vec<i32>,
/// }
///
/// impl KthLargest {
///     fn new(k: i32, nums: Vec<i32>) -> Self {
///         let mut kth = KthLargest { k: k as usize, v: Vec::new(), };
///         for n in nums {
///             kth.add(n);
///         }
///         kth
///     }
///
///     fn add(&mut self, val: i32) -> i32 {
///         if self.v.len() < self.k || Some(self.v.last().unwrap()) < Some(&val) {
///             self.v.push(val);
///             self.v.sort_unstable_by(|a, b| b.cmp(a));
///             if self.v.len() > self.k {
///                 self.v.pop();
///             }
///         }
///
///         *self.v.last().unwrap()
///     }
/// }
/// ```
///
pub struct KthLargest;
