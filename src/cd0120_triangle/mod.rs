//! Triangle [leetcode: triangle](https://leetcode.com/problems/triangle/)
//!
//! Given a triangle, find the minimum path sum from top to bottom. Each step you may move to adjacent numbers on the row below.
//!
//! For example, given the following triangle
//!
//! ```
//! [
//!      [2],
//!     [3,4],
//!    [6,5,7],
//!   [4,1,8,3]
//! ]
//! ```
//!
//! The minimum path sum from top to bottom is `11` (i.e., **2** + **3** + **5** + **1** = **11**).
//!
//! **Note:**
//!
//! Bonus point if you are able to do this using only *O(n)* extra space, where *n* is the total number of rows in the triangle.
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
/// impl Solution {
///     pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
///         if triangle.len() == 0 { return 0; }
///
///         let mut min = triangle.last().unwrap().clone();
///         for i in (0..triangle.len() - 1).rev() {
///             for j in 0..triangle[i].len() {
///                 min[j] = min[j].min(min[j + 1]) + triangle[i][j];
///             }
///         }
///        min[0]
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
/// * Memory: 2.7 MB
///
/// ```rust
/// impl Solution {
///     pub fn minimum_total(mut triangle: Vec<Vec<i32>>) -> i32 {
///         if triangle.len() == 0 { return 0; }
///
///         for i in (0..triangle.len() - 1).rev() {
///             for j in 0..triangle[i].len() {
///                 triangle[i][j] = triangle[i + 1][j].min(triangle[i + 1][j + 1]) + triangle[i][j];
///             }
///         }
///         triangle[0][0]
///     }
/// }
/// ```
///
pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
    if triangle.len() == 0 { return 0; }

    let mut min = triangle.last().unwrap().clone();
    for i in (0..triangle.len() - 1).rev() {
        for j in 0..triangle[i].len() {
            min[j] = min[j].min(min[j + 1]) + triangle[i][j];
        }
    }
   min[0]
}
