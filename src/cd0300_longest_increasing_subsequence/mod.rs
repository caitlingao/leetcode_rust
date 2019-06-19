//! Longest Increasing Subsequence [leetcode: longest_increasing_subsequence](https://leetcode.com/problems/longest-increasing-subsequence/)
//!
//! Given an unsorted array of integers, find the length of longest increasing subsequence.
//!
//! ***Example1:***
//!
//! ```
//! Input: [10,9,2,5,3,7,101,18]
//! Output: 4
//! Explanation: The longest increasing subsequence is [2,3,7,101], therefore the length is 4.
//! ```
//!
//! **Note:**
//! * There may be more than one LIS combination, it is only necessary for you to return the length.
//! * Your algorithm should run in O(n2) complexity.
//!
//! **Follow up:** Could you improve it to O(n log n) time complexity?

/// # Solutions
///
/// # Approach 1: Dynamic Programming
///
/// * Time complexity: O(nlogn)
///
/// * Space complexity: O(n)
///
/// * Runtime: 0 ms
/// * Memory: 2.5 MB
///
/// ```rust
/// impl Solution {
///     pub fn length_of_lis(nums: Vec<i32>) -> i32 {
///         if nums.len() <= 1 { return nums.len() as i32; }
///
///         let mut lis = vec![];
///         lis.push(nums[0]);
///
///         for i in 1..nums.len() {
///             match lis.binary_search(&nums[i]) {
///                 Ok(n) => (),
///                 Err(n) => {
///                     if n >= lis.len() { lis.push(nums[i]); } else { lis[n] = nums[i]; }
///                 },
///             }
///         }
///         lis.len() as i32
///     }
/// }
/// ```
///
/// # Approach 2: Dynamic Programming
///
/// * Time complexity: O(n2)
///
/// * Space complexity: O(n)
///
/// * Runtime: 12 ms
/// * Memory: 2.5 MB
///
/// ```rust
/// use std::cmp::max;
///
/// impl Solution {
///     pub fn length_of_lis(nums: Vec<i32>) -> i32 {
///         if nums.len() <= 1 { return nums.len() as i32; }
///
///         let mut dp = vec![1; nums.len()];
///         let mut max_lis = 1;
///
///         for i in 1..nums.len() {
///             let mut tmp_max = 0;
///             for j in 0..i {
///                 if nums[i] > nums[j] {
///                     tmp_max = max(tmp_max, dp[j]);
///                 }
///                 dp[i] = tmp_max + 1;
///             }
///             max_lis = max(max_lis, dp[i]);
///         }
///         max_lis
///     }
/// }
/// ```
///
/// # Approach 3: Dynamic Programming
///
/// * Time complexity: O(n2)
///
/// * Space complexity: O(n)
///
/// * Runtime: 8 ms
/// * Memory: 2.5 MB
///
/// ```rust
/// use std::cmp::max;
///
/// impl Solution {
///     pub fn length_of_lis(nums: Vec<i32>) -> i32 {
///         if nums.len() <= 1 { return nums.len() as i32; }
///
///         let mut dp = vec![1; nums.len()];
///         let mut max_lis = 1;
///
///         for i in 1..nums.len() {
///             for j in 0..i {
///                 if nums[i] > nums[j] {
///                     dp[i] = max(dp[i], dp[j] + 1);
///                 }
///             }
///             max_lis = max(max_lis, dp[i]);
///         }
///         max_lis
///     }
/// }
/// ```
///

pub fn length_of_lis(nums: Vec<i32>) -> i32 {
    if nums.len() <= 1 { return nums.len() as i32; }

    let mut dp = vec![1; nums.len()];
    let mut max_lis = 1;

    for i in 1..nums.len() {
        let mut tmp_max = 0;
        for j in 0..i {
            if nums[i] > nums[j] {
                tmp_max = i32::max(tmp_max, dp[j]);
            }
            dp[i] = tmp_max + 1;
        }
        max_lis = i32::max(max_lis, dp[i]);
    }
    max_lis
}
