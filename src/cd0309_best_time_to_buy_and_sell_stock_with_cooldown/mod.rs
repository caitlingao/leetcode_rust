//! Best Time to Buy and Sell Stock with Cooldown [leetcode: best_time_to_buy_and_sell_stock_with_cooldown](https://leetcode.com/problems/best-time-to-buy-and-sell-stock-with-cooldown/)
//!
//! Say you have an array for which the ith element is the price of a given stock on day *i*.
//!
//! Design an algorithm to find the maximum profit. You may complete as many transactions as you like (ie, buy one and sell one share of the stock multiple times) with the following restrictions:
//!
//! * You may not engage in multiple transactions at the same time (ie, you must sell the stock before you buy again).
//! * After you sell your stock, you cannot buy stock on next day. (ie, cooldown 1 day)
//!
//! ***Example1:***
//!
//! ```
//! Input: [1,2,3,0,2]
//! Output: 3
//! Explanation: transactions = [buy, sell, cooldown, buy, sell]
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
/// use std::cmp::max;
///
/// impl Solution {
///     pub fn max_profit(prices: Vec<i32>) -> i32 {
///         if prices.len() < 2 { return 0; }
///
///         let cooldown = 1;
///         let mut profits = vec![0; prices.len()];
///         let mut max_profit = 0;
///         let mut tmp_max = profits[0] - prices[0];
///
///         for i in 1..prices.len() {
///             profits[i] = max(profits[i - 1], tmp_max + prices[i]);
///             tmp_max    = max(tmp_max, if i > cooldown { profits[i - 1 - cooldown] } else { profits[i - 1] } - prices[i]);
///             max_profit = max(profits[i], max_profit);
///         }
///         max_profit
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
/// * Memory: 2.5 MB
///
/// ```rust
/// use std::cmp::max;
///
/// impl Solution {
///     pub fn max_profit(prices: Vec<i32>) -> i32 {
///         if prices.len() < 2 { return 0; }
///
///         let cooldown = 1;
///         let mut sell = vec![0; prices.len()];
///         let mut buy = vec![0; prices.len()];
///         buy[0] = -prices[0];
///
///         for i in 1..prices.len() {
///             sell[i] = max(sell[i - 1], buy[i - 1] + prices[i]);
///             buy[i]  = max(buy[i - 1], if i > cooldown { sell[i - 1 - cooldown] } else { sell[i - 1] } - prices[i]);
///         }
///         sell[prices.len() - 1]
///     }
/// }
/// ```
///

pub fn max_profit(prices: Vec<i32>) -> i32 {
    if prices.len() < 2 { return 0; }

    let cooldown = 1;
    let mut profits = vec![0; prices.len()];
    let mut max_profit = 0;
    let mut tmp_max = profits[0] - prices[0];

    for i in 1..prices.len() {
        profits[i] = i32::max(profits[i - 1], tmp_max + prices[i]);
        tmp_max    = i32::max(tmp_max, if i > cooldown { profits[i - 1 - cooldown] } else { profits[i - 1] } - prices[i]);
        max_profit = i32::max(profits[i], max_profit);
    }
    max_profit
}
