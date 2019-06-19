//! Climbing Stairs [leetcode: climbing_stairs](https://leetcode.com/problems/climbing-stairs/)
//!
//! You are climbing a stair case. It takes n steps to reach to the top.
//!
//! Each time you can either climb 1 or 2 steps. In how many distinct ways can you climb to the top?
//!
//! **Note:** Given `n` will be a positive integer.
//!
//! **Example 1:**
//! ```
//! Input: 2
//! Output: 2
//! Explanation: There are two ways to climb to the top.
//! 1. 1 step + 1 step
//! 2. 2 steps
//! ```
//!
//! **Example 2:**
//! ```
//! Input: 3
//! Output: 3
//! Explanation: There are three ways to climb to the top.
//! 1. 1 step + 1 step + 1 step
//! 2. 1 step + 2 steps
//! 3. 2 steps + 1 step
//! ```
//!
/// # Solutions
///
/// # Approach 1: Dynamic Programming
///
/// * Time complexity: O(n)
///
/// * Space complexity: O(n)
///
/// * Runtime: 0 ms
/// * Memory: 2.3 MB
///
/// ```rust
/// impl Solution {
///     pub fn climb_stairs(n: i32) -> i32 {
///         match n {
///             0 | 1 | 2 => n,
///             k => {
///                 let mut count = vec![0; (n + 1) as usize];
///                 count[1] = 1;
///                 count[2] = 2;
///                 for i in 3..=k as usize { count[i] = count[i - 1] + count[i - 2]; }
///                 count[k as usize]
///             }
///         }
///     }
/// }
/// ```
///
/// # Approach 2: Fibonacci Number
///
/// * Time complexity: O(n)
///
/// * Space complexity: O(1)
///
/// * Runtime: 0 ms
/// * Memory: 2.4 MB
///
/// ```rust
/// impl Solution {
///     pub fn climb_stairs(n: i32) -> i32 {
///         match n {
///             0 | 1 | 2 => n,
///             k => (2..k).fold((1, 2), |acc, __| (acc.1, acc.0 + acc.1)).1,
///         }
///     }
/// }
/// ```
///
/// # Approach 3: Dynamic Programming
///
/// * Time complexity: O(2n)
///
/// * Space complexity: O(n)
///
/// * Runtime: 0 ms
/// * Memory: 2.5 MB
///
/// ```rust
/// impl Solution {
///     pub fn climb_stairs(n: i32) -> i32 {
///         let steps = vec![1, 2];
///         let mut dp = vec![0; (n + 1) as usize];
///         dp[0] = 1;
///
///         for i in 1..=n as usize {
///             for j in 0..steps.len() {
///                 if i >= steps[j] { dp[i] += dp[i - steps[j]]; }
///             }
///         }
///
///         *dp.last().unwrap()
///     }
/// }
/// ```
pub fn climb_stairs(n: i32) -> i32 {
    match n {
        0 | 1 | 2 => n,
        k => {
            let mut count = vec![0; (n + 1) as usize];
            count[1] = 1;
            count[2] = 2;
            for i in 3..=k as usize { count[i] = count[i - 1] + count[i - 2]; }
            count[k as usize]
        }
    }
}
