//! Edit Distance [leetcode: edit_distance](https://leetcode.com/problems/edit-distance/)
//!
//! Given two words *word1* and *word2*, find the minimum number of operations required to convert *word1* to *word2*.
//!
//! You have the following 3 operations permitted on a word:
//! 1. Insert a character
//! 2. Delete a character
//! 3. Replace a character
//!
//! **Example1:**
//!
//! ```
//! Input: word1 = "horse", word2 = "ros"
//! Output: 3
//! Explanation:
//! horse -> rorse (replace 'h' with 'r')
//! rorse -> rose (remove 'r')
//! rose -> ros (remove 'e')
//! ```
//!
//! **Example2:**
//!
//! ```
//! Input: word1 = "intention", word2 = "execution"
//! Output: 5
//! Explanation:
//! intention -> inention (remove 't')
//! inention -> enention (replace 'i' with 'e')
//! enention -> exention (replace 'n' with 'x')
//! exention -> exection (replace 'n' with 'c')
//! exection -> execution (insert 'u')
//! ```
//!

/// # Solutions
///
/// # Approach 1: Dynamic Programming
///
/// * Time complexity: O(m*n)
///
/// * Space complexity: O(m*n)
///
/// * Runtime: 4 ms
/// * Memory: 4.5 MB
///
/// ```rust
/// impl Solution {
///     pub fn min_distance(word1: String, word2: String) -> i32 {
///         let word1_bytes = word1.into_bytes();
///         let word2_bytes = word2.into_bytes();
///         let m = word1_bytes.len();
///         let n = word2_bytes.len();
///         let mut dp = vec![vec![0; n + 1]; m + 1];
///
///         // number of deletions to reach word2[0]
///         for i in 0..=m { dp[i][0] = i; }
///         // to construct word2[j] just with word1[0] need to j insertions
///         for j in 0..=n { dp[0][j] = j; }
///
///         for i in 1..=m {
///             for j in 1..=n {
///                 // we can obtain word2[0..j] from word1[0..i] either by
///                 // deleting the ith charactor. knowing eg: dp[i - 1][j]
///                 // inserting the jth charactor. knowing eg: dp[i][j - 1]
///                 // or replate ith charactor with jth charactor. knowing eg: dp[i - 1][j - 1]
///                 dp[i][j] = (dp[i - 1][j - 1] + (word1_bytes[i - 1] != word2_bytes[j - 1]) as usize)
///                 .min(dp[i - 1][j] + 1)
///                 .min(dp[i][j - 1] + 1);
///             }
///         }
///         dp[m][n] as i32
///     }
/// }
/// ```
///
/// # Approach 2: Dynamic Programming
///
/// * Time complexity: O(m*n)
///
/// * Space complexity: O(m*n)
///
/// * Runtime: 4 ms
/// * Memory: 4.5 MB
///
/// ```rust
/// impl Solution {
///     pub fn min_distance(word1: String, word2: String) -> i32 {
///         let word1_bytes = word1.into_bytes();
///         let word2_bytes = word2.into_bytes();
///         let m = word1_bytes.len();
///         let n = word2_bytes.len();
///         let mut dp = vec![vec![0; n + 1]; m + 1];
///
///         // number of deletions to reach word2[0]
///         for i in 0..=m { dp[i][0] = i; }
///         // to construct word2[j] just with word1[0] need to j insertions
///         for j in 0..=n { dp[0][j] = j; }
///
///         for i in 1..=m {
///             for j in 1..=n {
///                 if word1_bytes[i - 1] == word2_bytes[j - 1] {
///                     dp[i][j] = dp[i - 1][j - 1];
///                 } else {
///                     // we can obtain word2[0..j] from word1[0..i] either by
///                     // deleting the ith charactor. knowing eg: dp[i - 1][j]
///                     // inserting the jth charactor. knowing eg: dp[i][j - 1]
///                     // or replate ith charactor with jth charactor. knowing eg: dp[i - 1][j - 1]
///                     dp[i][j] = dp[i - 1][j].min(dp[i][j - 1]).min(dp[i - 1][j - 1]) + 1;
///                 }
///             }
///         }
///         dp[m][n] as i32
///     }
/// }
/// ```
///

pub fn min_distance(word1: String, word2: String) -> i32 {
    let word1_bytes = word1.into_bytes();
    let word2_bytes = word2.into_bytes();
    let m = word1_bytes.len();
    let n = word2_bytes.len();
    let mut dp = vec![vec![0; n + 1]; m + 1];

    // number of deletions to reach word2[0]
    for i in 0..=m { dp[i][0] = i; }
    // to construct word2[j] just with word1[0] need to j insertions
    for j in 0..=n { dp[0][j] = j; }

    for i in 1..=m {
        for j in 1..=n {
            // we can obtain word2[0..j] from word1[0..i] either by
            // deleting the ith charactor. knowing eg: dp[i - 1][j]
            // inserting the jth charactor. knowing eg: dp[i][j - 1]
            // or replate ith charactor with jth charactor. knowing eg: dp[i - 1][j - 1]
            dp[i][j] = (dp[i - 1][j - 1] + (word1_bytes[i - 1] != word2_bytes[j - 1]) as usize)
            .min(dp[i - 1][j] + 1)
            .min(dp[i][j - 1] + 1);
        }
    }
    dp[m][n] as i32
}
