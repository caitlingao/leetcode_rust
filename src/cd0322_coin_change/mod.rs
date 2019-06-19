//! Coin Change [leetcode: coin_change](https://leetcode.com/problems/coin-change/)
//!
//! You are given coins of different denominations and a total amount of money *amount*. Write a function to compute the fewest number of coins that you need to make up that amount. If that amount of money cannot be made up by any combination of the coins, return `-1`.
//!
//! ***Example1:***
//!
//! ```
//! Input: coins = [1, 2, 5], amount = 11
//! Output: 3
//! Explanation: 11 = 5 + 5 + 1
//! ```
//!
//! ***Example2:***
//!
//! ```
//! Input: coins = [2], amount = 3
//! Output: -1
//! ```
//!
//! **Note:**
//! You may assume that you have an infinite number of each kind of coin.
//!

/// # Solutions
///
/// # Approach 1: Dynamic Programming
///
/// * Time complexity: O(s*n)
///
/// * Space complexity: O(s)
///
/// * Runtime: 4 ms
/// * Memory: 3 MB
///
/// ```rust
/// impl Solution {
///     pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
///         let mut dp = vec![amount + 1; (amount + 1) as usize];
///         dp[0] = 0;
///
///         for i in 1..=amount {
///             for &coin in coins.iter() {
///                 if i >= coin {
///                     dp[i as usize] = i32::min(dp[i as usize], dp[(i - coin) as usize] + 1);
///                 }
///             }
///         }
///         let last = *dp.last().unwrap();
///         if last > amount { return -1; } else { return last; }
///     }
/// }
/// ```
///
/// # Approach 2: Dynamic Programming
///
/// * Time complexity: O(s*n)
///
/// * Space complexity: O(s)
///
/// * Runtime: 4 ms
/// * Memory: 4 MB
///
/// ```rust
/// impl Solution {
///     pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
///         let mut dp = vec![-1; (amount+1) as usize];
///         dp[0] = 0;
///
///         for i in 1..=amount as usize {
///             dp[i] = std::i32::MAX;
///             for &coin in coins.iter() {
///                 if i >= coin as usize && dp[i - coin as usize] != -1 {
///                     dp[i] = i32::min(dp[i], dp[i - coin as usize] + 1);
///                 }
///             }
///             if dp[i] == std::i32::MAX { dp[i] = -1; }
///         }
///         *dp.last().unwrap()
///     }
/// }
/// ```
///

pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
    let mut dp = vec![amount + 1; (amount + 1) as usize];
    dp[0] = 0;

    for i in 1..=amount {
        for &coin in coins.iter() {
            if i >= coin {
                dp[i as usize] = i32::min(dp[i as usize], dp[(i - coin) as usize] + 1);
            }
        }
    }
    let last = *dp.last().unwrap();
    if last > amount { return -1; } else { return last; }
}
