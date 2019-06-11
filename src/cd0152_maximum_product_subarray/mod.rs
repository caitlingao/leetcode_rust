//! Maximum Product Subarray [leetcode: maximum_product_subarray](https://leetcode.com/problems/maximum-product-subarray/)
//!
//! Given an integer array `nums`, find the contiguous subarray within an array (containing at least one number) which has the largest product.
//!
//! **Example 1:**
//!
//! ```
//! Input: [2,3,-2,4]
//! Output: 6
//! Explanation: [2,3] has the largest product 6.
//! ```
//!
//! **Example 2:**
//!
//! ```
//! Input: [-2,0,-1]
//! Output: 0
//! Explanation: The result cannot be 2, because [-2,-1] is not a subarray.
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
/// * Memory: 2.6 MB
///
/// ```rust
/// use std::cmp;
///
/// impl Solution {
///     pub fn max_product(nums: Vec<i32>) -> i32 {
///         if nums.len() == 0 { return 0; }
///
///         let mut dp = vec![vec![0; 2]; 2]; // dp[x][0] max value, dp[x][1] min value
///         let mut result = nums[0];
///
///         dp[0][0] = nums[0];
///         dp[0][1] = nums[0];
///
///         for i in 1..nums.len() {
///             let (x, y) = (i % 2, (i - 1) % 2); // x, y is 0 or 1
///
///             if nums[i] >= 0 {
///                 dp[x][0] = cmp::max(dp[y][0] * nums[i], nums[i]);
///                 dp[x][1] = cmp::min(dp[y][1] * nums[i], nums[i]);
///             } else {
///                 dp[x][0] = cmp::max(dp[y][1] * nums[i], nums[i]);
///                 dp[x][1] = cmp::min(dp[y][0] * nums[i], nums[i]);
///             }
///
///             // simplify if else like this
///             // dp[x][0] = vec![dp[y][0] * nums[i], dp[y][1] * nums[i], nums[i]].iter().max().unwrap().clone();
///             // dp[x][1] = vec![dp[y][0] * nums[i], dp[y][1] * nums[i], nums[i]].iter().min().unwrap().clone();
///
///             result = cmp::max(dp[x][0], result);
///         }
///
///         result
///     }
/// }
/// ```
///
/// # Approach 2: Dynamic Programming
///
/// * Time complexity: O(n)
///
/// * Space complexity: O(1)
///
/// * Runtime: 0 ms
/// * Memory: 2.5 MB
///
/// ```rust
/// impl Solution {
///     pub fn max_product(nums: Vec<i32>) -> i32 {
///         if nums.is_empty() { return 0; }
///
///         let mut cur_max = nums[0];
///         let mut cur_min = nums[0];
///         let mut result = nums[0];
///
///         for &num in nums[1..].into_iter() {
///             if num >= 0 {
///                 cur_max = i32::max(cur_max * num, num);
///                 cur_min = i32::min(cur_min * num, num);
///             } else {
///                 let tmp = cur_max;
///                 cur_max = i32::max(cur_min * num, num);
///                 cur_min = i32::min(tmp * num, num);
///             }
///
///             result = i32::max(cur_max, result);
///         }
///
///         result
///     }
/// }
/// ```
///
/// # Approach 3: Dynamic Programming
///
/// * Time complexity: O(n)
///
/// * Space complexity: O(1)
///
/// * Runtime: 0 ms
/// * Memory: 2.5 MB
///
/// ```rust
/// use std::mem;
/// use std::cmp;
/// impl Solution {
///     pub fn max_product(nums: Vec<i32>) -> i32 {
///         let mut min = nums[0];
///         let mut max = nums[0];
///         let mut res = nums[0];
///
///         for i in 1..nums.len() {
///             if nums[i] < 0 {
///                 mem::swap(&mut min, &mut max);
///             }
///
///             min = cmp::min(nums[i], min*nums[i]);
///             max = cmp::max(nums[i], max*nums[i]);
///
///             res = cmp::max(res, max);
///         }
///
///         return res;
///     }
/// }
/// ```
///
pub fn max_product(nums: Vec<i32>) -> i32 {
    if nums.is_empty() { return 0; }

    let mut cur_max = nums[0];
    let mut cur_min = nums[0];
    let mut result = nums[0];

    for &num in nums[1..].into_iter() {
        if num >= 0 {
            cur_max = i32::max(cur_max * num, num);
            cur_min = i32::min(cur_min * num, num);
        } else {
            let tmp = cur_max;
            cur_max = i32::max(cur_min * num, num);
            cur_min = i32::min(tmp * num, num);
        }

        result = i32::max(cur_max, result);
    }

    result
}
