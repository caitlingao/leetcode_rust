//! Best Time to Buy and Sell Stock with Transaction Fee [leetcode: best_time_to_buy_and_sell_stock_with_transaction_fee](https://leetcode.com/problems/best-time-to-buy-and-sell-stock-with-transaction-fee/)
//!
//! Your are given an array of integers `prices`, for which the `i`-th element is the price of a given stock on day `i`; and a non-negative integer fee representing a transaction `fee`.
//!
//! You may complete as many transactions as you like, but you need to pay the transaction fee for each transaction. You may not buy more than 1 share of a stock at a time (ie. you must sell the stock share before you buy again.)
//!
//! Return the maximum profit you can make.
//!
//! ***Example1:***
//!
//! ```
//! Input: prices = [1, 3, 2, 8, 4, 9], fee = 2
//! Output: 8
//! Explanation: The maximum profit can be achieved by:
//! Buying at prices[0] = 1
//! Selling at prices[3] = 8
//! Buying at prices[4] = 4
//! Selling at prices[5] = 9
//! The total profit is ((8 - 1) - 2) + ((9 - 4) - 2) = 8.
//! ```
//!
//! **Note:**
//!
//! * `0 < prices.length <= 50000.`
//! * `0 < prices[i] < 50000.`
//! * `0 <= fee < 50000.`

/// # Solutions
///
/// # Approach 1: Dynamic Programming
///
/// * Time complexity: O(n)
///
/// * Space complexity: O(n)
///
/// * Runtime: 8 ms
/// * Memory: 3.7 MB
///
/// ```rust
/// use std::cmp::max;
///
/// impl Solution {
///     pub fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
///         if prices.len() < 2 || fee >= 50000 { return 0; }
///
///         let mut profits = vec![0; prices.len()];
///         let mut max_profit = 0;
///         let mut tmp_max = profits[0] - prices[0];
///
///         for i in 1..prices.len() {
///             profits[i] = max(profits[i - 1], tmp_max + prices[i] - fee);
///             tmp_max = max(tmp_max, profits[i - 1] - prices[i]);
///             max_profit = max(max_profit, profits[i]);
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
/// * Runtime: 8 ms
/// * Memory: 3.6 MB
///
/// ```rust
///use std::cmp::max;
///
///impl Solution {
///    pub fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
///        if prices.len() < 2 || fee >= 50000 { return 0; }
///
///        let mut sell = vec![0; prices.len()];
///        let mut buy = vec![0; prices.len()];
///        buy[0] = -prices[0];
///
///        for i in 1..prices.len() {
///            sell[i] = max(sell[i - 1], buy[i - 1] + prices[i] - fee);
///            buy[i] = max(buy[i - 1], sell[i - 1] - prices[i]);
///        }
///        sell[prices.len() - 1]
///    }
///}
/// ```
///
/// # Approach 3: Dynamic Programming
///
/// * Time complexity: O(n)
///
/// * Space complexity: O(n)
///
/// * Runtime: 12 ms
/// * Memory: 3.7 MB
///
/// ```rust
/// use std::cmp::max;
///
/// impl Solution {
///     pub fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
///         if prices.len() < 2 || fee >= 50000 { return 0; }
///
///         let mut cash = 0;
///         let mut hold = -prices[0];
///         for i in 1..prices.len() {
///             cash = max(cash, hold + prices[i] - fee);
///             hold = max(hold, cash - prices[i]);
///         }
///         cash
///     }
/// }
/// ```
///

pub fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
    if prices.len() < 2 || fee >= 50000 { return 0; }

    let mut cash = 0;
    let mut hold = -prices[0];
    for i in 1..prices.len() {
        cash = i32::max(cash, hold + prices[i] - fee);
        hold = i32::max(hold, cash - prices[i]);
    }
    cash
}
