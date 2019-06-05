//! Counting Bits [counting_bits](https://leetcode.com/problems/counting-bits/)
//!
//! Given a non negative integer number **num**. For every numbers i in the range **0 ≤ i ≤ num** calculate the number of 1's in their binary representation and return them as an array.
//!
//! ***Example1:***
//!
//! ```
//! Input: 2
//! Output: [0,1,1]
//! ```
//!
//! ***Example2:***
//!
//! ```
//! Input: 5
//! Output: [0,1,1,2,1,2]
//! ```
//!
//! **Follow up::**
//!
//! * It is very easy to come up with a solution with run time **O(n*sizeof(integer))**. But can you do it in linear time **O(n)** /possibly in a single pass?
//! * Space complexity should be **O(n)**.
//! * Can you do it like a boss? Do it without using any builtin function like **__builtin_popcount** in c++ or in any other language.

/// # Solutions
///
/// # Approach 1: Iteration
///
/// * Time complexity: O(n)
///
/// * Space complexity: O(1)
///
/// * Runtime: 4 ms
/// * Memory: 2.9 MB
///
/// ```rust
/// impl Solution {
///     pub fn count_bits(num: i32) -> Vec<i32> {
///         let mut bits = vec![0; (num + 1) as usize];
///         for i in 1..=num as usize {
///             bits[i] = bits[i & (i - 1)] + 1;
///         }
///         bits
///     }
/// }
/// ```
///
/// # Approach 2: Iteration
///
/// * Time complexity: O(n)
///
/// * Space complexity: O(1)
///
/// * Runtime: 12 ms
/// * Memory: 2.9 MB
///
/// ```rust
/// impl Solution {
///     pub fn count_bits(num: i32) -> Vec<i32> {
///         let mut result = vec![];
///         for i in 0..=num {
///             let mut j = i;
///             let mut count = 0;
///             while j != 0 {
///                 count += 1;
///                 j &= (j - 1);
///             }
///             result.push(count);
///         }
///         result
///     }
/// }
/// ```
///
pub fn count_bits(num: i32) -> Vec<i32> {
    let mut bits = vec![0; (num + 1) as usize];
    for i in 1..=num as usize {
        bits[i] = bits[i & (i - 1)] + 1;
    }
    bits
}
