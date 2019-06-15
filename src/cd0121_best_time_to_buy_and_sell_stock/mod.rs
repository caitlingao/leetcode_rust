//! Best Time to Buy and Sell Stock [leetcode: best_time_to_buy_and_sell_stock](https://leetcode.com/problems/best-time-to-buy-and-sell-stock/)
//!
//! Say you have an array for which the ith element is the price of a given stock on day *i*.
//!
//! If you were only permitted to complete at most one transaction (i.e., buy one and sell one share of the stock), design an algorithm to find the maximum profit.
//!
//! Note that you cannot sell a stock before you buy one.
//!
//! ***Example1:***
//!
//! ```
//! Input: [7,1,5,3,6,4]
//! Output: 5
//! Explanation: Buy on day 2 (price = 1) and sell on day 5 (price = 6), profit = 6-1 = 5.
//!              Not 7-1 = 6, as selling price needs to be larger than buying price.
//! ```
//!
//! ***Example2:***
//!
//! ```
//! Input: [7,6,4,3,1]
//! Output: 0
//! Explanation: In this case, no transaction is done, i.e. max profit = 0.
//! ```
//!

/// # Solutions
///
/// # Approach 1: Dynamic Programming
///
/// * Time complexity: O(3n)
///
/// * Space complexity: O(n)
///
/// * Runtime: 0 ms
/// * Memory: 3.6 MB
///
/// ```rust
/// use std::cmp;
///
/// impl Solution {
///     pub fn max_profit(prices: Vec<i32>) -> i32 {
///         if prices.len() < 2 { return 0; }
///
///         let mut result  = 0;
///         let mut profits = vec![vec![0; 3]; prices.len()];
///         profits[0][1] = -prices[0];
///
///         for i in 1..prices.len() {
///             profits[i][0] = profits[i - 1][0];
///             profits[i][1] = cmp::max(profits[i - 1][1], profits[i - 1][0] - prices[i]);
///             profits[i][2] = profits[i - 1][1] + prices[i];
///
///             result = cmp::max(result, cmp::max(profits[i][0], cmp::max(profits[i][1], profits[i][2])));
///         }
///         result
///     }
/// }
/// ```
///
/// # Approach 2: Dynamic Programming
///
/// * Time complexity: O(n)
///
/// * Space complexity: O(n)
///
/// * Runtime: 0 ms
/// * Memory: 2.7 MB
///
/// ```rust
/// impl Solution {
///     pub fn max_profit(prices: Vec<i32>) -> i32 {
///         if prices.len() < 2 { return 0; }
///
///         let mut profits = vec![0; prices.len()];
///         profits[0] = -prices[0];
///         for i in 1..prices.len() {
///             if profits[i - 1] < 0 { profits[ i - 1] = 0; }
///
///             profits[i] = profits[i - 1] - prices[i - 1] + prices[i];
///         }
///
///         profits.sort();
///         *profits.last().unwrap()
///     }
/// }
/// ```
///
/// # Approach 3: Dynamic Programming
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
///     pub fn max_profit(prices: Vec<i32>) -> i32 {
///         if prices.len() < 2 { return 0; }
///
///         let mut profits = vec![0; prices.len()];
///         let mut max_profit = 0;
///         let mut tmp_min = prices[0];
///         for i in 1..prices.len() {
///             profits[i] = cmp::max(profits[i - 1], prices[i] - tmp_min);
///             tmp_min = cmp::min(tmp_min, prices[i]);
///             max_profit = cmp::max(profits[i], max_profit);
///         }
///
///         max_profit
///     }
/// }
/// ```
///
/// # Approach 4: Dynamic Programming
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
///     pub fn max_profit(prices: Vec<i32>) -> i32 {
///         if prices.len() < 2 { return 0; }
///
///         let mut profits = vec![0; 2];
///         let mut max_profit = 0;
///         let mut tmp_min = prices[0];
///         for i in 1..prices.len() {
///             let (x, y) = (i % 2, (i - 1) % 2);
///             profits[x] = cmp::max(profits[y], prices[i] - tmp_min);
///             tmp_min = cmp::min(tmp_min, prices[i]);
///             max_profit = cmp::max(profits[x], max_profit);
///         }
///
///         max_profit
///     }
/// }
/// ```
///
pub fn max_profit(prices: Vec<i32>) -> i32 {
    if prices.len() < 2 { return 0; }

    let mut profits = vec![0; prices.len()];
    profits[0] = -prices[0];
    for i in 1..prices.len() {
        if profits[i - 1] < 0 { profits[ i - 1] = 0; }

        profits[i] = profits[i - 1] - prices[i - 1] + prices[i];
    }

    profits.sort();
    *profits.last().unwrap()
}
