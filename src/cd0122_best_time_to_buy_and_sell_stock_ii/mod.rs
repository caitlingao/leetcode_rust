//! Best Time to Buy and Sell Stock II [leetcode: best_time_to_buy_and_sell_stock_II](https://leetcode.com/problems/best-time-to-buy-and-sell-stock-ii/)
//!
//! Say you have an array for which the ith element is the price of a given stock on day *i*.
//!
//! Design an algorithm to find the maximum profit. You may complete as many transactions as you like (i.e., buy one and sell one share of the stock multiple times).
//!
//! **Note**: You may not engage in multiple transactions at the same time (i.e., you must sell the stock before you buy again).
//!
//! ***Example1:***
//!
//! ```
//! Input: [7,1,5,3,6,4]
//! Output: 7
//! Explanation: Buy on day 2 (price = 1) and sell on day 3 (price = 5), profit = 5-1 = 4.
//!             Then buy on day 4 (price = 3) and sell on day 5 (price = 6), profit = 6-3 = 3.
//! ```
//!
//! ***Example2:***
//!
//! ```
//! Input: [1,2,3,4,5]
//! Output: 4
//! Explanation: Buy on day 1 (price = 1) and sell on day 5 (price = 5), profit = 5-1 = 4.
//!              Note that you cannot buy on day 1, buy on day 2 and sell them later, as you are
//!             engaging multiple transactions at the same time. You must sell before buying again.
//! ```
//!
//! ***Example3:***
//!
//! ```
//! Input: [7,6,4,3,1]
//! Output: 0
//! Explanation: In this case, no transaction is done, i.e. max profit = 0.
//! ```

/// # Solutions
///
/// # Approach 1: Iteration
///
/// * Time complexity: O(0)
///
/// * Space complexity: O(1)
///
/// * Runtime: 0 ms
/// * Memory: 2.6 MB
///
/// ```rust
/// impl Solution {
///     pub fn max_profit(prices: Vec<i32>) -> i32 {
///         if prices.len() <= 1 { return 0; }
///
///         let mut sum = 0;
///         for i in 0..prices.len() - 1 {
///             if prices[i] < prices[i+1] { sum += prices[i+1] - prices[i]; }
///         }
///         sum
///     }
/// }
/// ```
///
/// # Approach 2: Iteration
///
/// * Time complexity: O(0)
///
/// * Space complexity: O(1)
///
/// * Runtime: 0 ms
/// * Memory: 2.6 MB
///
/// ```rust
/// use std::cmp::max;
///
/// impl Solution {
///     pub fn max_profit(prices: Vec<i32>) -> i32 {
///         if prices.len() <= 1 { return 0; }
///
///         let mut max_profit = 0;
///         for i in 1..prices.len() {
///             max_profit += max(0, prices[i] - prices[i - 1]);
///         }
///         max_profit
///     }
/// }
/// ```
///
/// # Approach 3: Dynamic Programming
///
/// * Time complexity: O(0)
///
/// * Space complexity: O(1)
///
/// * Runtime: 0 ms
/// * Memory: 2.6 MB
///
/// ```rust
/// use std::cmp::max;
///
/// impl Solution {
///     pub fn max_profit(prices: Vec<i32>) -> i32 {
///         if prices.len() <= 1 { return 0; }
///
///         let mut profits = vec![0; prices.len()];
///         let mut max_profit = 0;
///         let mut tmp_max = -prices[0];
///
///         for i in 1..prices.len() {
///             profits[i] = max(profits[i - 1], tmp_max + prices[i]);
///             tmp_max = max(tmp_max, profits[i] - prices[i]);
///             max_profit = max(max_profit, profits[i]);
///         }
///
///         max_profit
///     }
/// }
/// ```
///
pub fn max_profit(prices: Vec<i32>) -> i32 {
    if prices.len() <= 1 { return 0; }

    let mut sum = 0;
    for i in 0..prices.len() - 1 {
        if prices[i] < prices[i+1] { sum += prices[i+1] - prices[i]; }
    }
    sum
}
