//! N-Queens [leetcode: n_queens](https://leetcode.com/problems/n-queens/)
//!
//! The *n*-queens puzzle is the problem of placing *n* queens on an *n×n* chessboard such that no two queens attack each other.
//!
//! Given an integer *n*, return all distinct solutions to the n-queens puzzle.
//!
//! Each solution contains a distinct board configuration of the *n*-queens' placement, where `'Q'` and `'.'` both indicate a queen and an empty space respectively.
//!
//! ***Example:***
//!
//! ```
//! Input: 4
//! Output: [
//!  [".Q..",  // Solution 1
//!   "...Q",
//!   "Q...",
//!   "..Q."],
//!
//!  ["..Q.",  // Solution 2
//!   "Q...",
//!   "...Q",
//!   ".Q.."]
//! ]
//! Explanation: There exist two distinct solutions to the 4-queens puzzle as shown above.
//! ```
//!

/// # Solutions
///
/// # Approach 1: DFS
///
/// * Time complexity:
///
/// * Space complexity:
///
/// * Runtime: 4 ms
/// * Memory: 2.8 MB
///
/// ```rust
/// impl Solution {
///     pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
///         if n < 1 { return vec![]; }
///
///         let mut result = vec![];
///         let mut current_queens = vec![];
///         let mut cols = vec![];
///         let mut xy_sum = vec![];
///         let mut xy_sub = vec![];
///         let row = 0;
///
///         Self::_dfs(n, &mut result, &mut current_queens, row, &mut cols, &mut xy_sum, &mut xy_sub);
///
///         result
///     }
///
///     pub fn _dfs(n: i32, result: &mut Vec<Vec<String>>, current_queens: &mut Vec<i32>, row: i32, cols: &mut Vec<i32>, xy_sum: &mut Vec<i32>, xy_sub: &mut Vec<i32>) {
///         if row >= n {
///             result.push(Self::matrix(current_queens, n));
///             return;
///         }
///
///         for col in 0..n {
///             if cols.contains(&col) || xy_sum.contains(&(row + col)) || xy_sub.contains(&(row - col)) {
///                 continue;
///             }
///
///             cols.push(col);
///             xy_sum.push(row + col);
///             xy_sub.push(row - col);
///             Self::_dfs(n, result, &mut [current_queens.clone(), [col].to_vec()].concat(), row + 1, cols, xy_sum, xy_sub);
///
///             cols.retain(|&x| x != col);
///             xy_sum.retain(|&x| x != (row + col));
///             xy_sub.retain(|&x| x != (row - col));
///         }
///     }
///
///     pub fn matrix(queens: &mut Vec<i32>, n: i32) -> Vec<String> {
///         let mut arr = vec![];
///         let queens_len = queens.len();
///         for i in queens {
///             let mut char_vector = vec!['.'; n as usize];
///             char_vector[*i as usize] = 'Q';
///             let str: String = char_vector.into_iter().collect();
///             arr.push(str);
///         }
///
///         arr
///     }
/// }
/// ```
///
/// # Approach 2: DFS
///
/// * Time complexity:
///
/// * Space complexity:
///
/// * Runtime: 0 ms
/// * Memory: 2.8 MB
///
/// ```rust
/// impl Solution {
///     pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
///         let mut board = vec![vec!['.'; n as usize]; n as usize];
///         let mut solution = vec![];
///         Self::schedule_queens(&mut board, &mut solution, n as usize, 0);
///         solution
///     }
///
///     fn schedule_queens(board: &mut Vec<Vec<char>>, solution: &mut Vec<Vec<String>>, len: usize, row: usize) {
///         for col in 0..len {
///             if !Self::collision(&board, len, row, col) {
///                 board[row][col] = 'Q';
///                 if row == len - 1 {
///                     solution.push(board.iter().map(|vec| vec.iter().collect()).collect());
///                 } else {
///                     Self::schedule_queens(board, solution, len, row+1);
///                 }
///                 board[row][col] = '.';
///             }
///         }
///     }
///
///     #[inline(always)]
///     fn collision(board: &Vec<Vec<char>>, len: usize, row: usize, col: usize) -> bool {
///         for i in 0..row {
///             if board[i][col] == 'Q' { return true }
///         }
///         let (mut i, mut j) = (row as i32 - 1, col as i32 - 1);
///         while i >= 0 && j >= 0 {
///             if board[i as usize][j as usize] == 'Q' { return true }
///             i -= 1; j -= 1;
///         }
///         let (mut i, mut j) = (row as i32 - 1, col as i32 + 1);
///         while i >= 0 && j < len as i32 {
///             if board[i as usize][j as usize] == 'Q' { return true}
///             i -= 1; j += 1;
///         }
///         false
///     }
/// }
///

pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
    if n < 1 { return vec![]; }

    let mut result = vec![];
    let mut current_queens = vec![];
    let mut cols = vec![];
    let mut xy_sum = vec![];
    let mut xy_sub = vec![];
    let row = 0;

    _dfs(n, &mut result, &mut current_queens, row, &mut cols, &mut xy_sum, &mut xy_sub);

    result
}

pub fn _dfs(n: i32, result: &mut Vec<Vec<String>>, current_queens: &mut Vec<i32>, row: i32, cols: &mut Vec<i32>, xy_sum: &mut Vec<i32>, xy_sub: &mut Vec<i32>) {
    if row >= n {
        result.push(matrix(current_queens, n));
        return;
    }

    for col in 0..n {
        if cols.contains(&col) || xy_sum.contains(&(row + col)) || xy_sub.contains(&(row - col)) {
            continue;
        }

        cols.push(col);
        xy_sum.push(row + col);
        xy_sub.push(row - col);
        _dfs(n, result, &mut [current_queens.clone(), [col].to_vec()].concat(), row + 1, cols, xy_sum, xy_sub);

        cols.retain(|&x| x != col);
        xy_sum.retain(|&x| x != (row + col));
        xy_sub.retain(|&x| x != (row - col));
    }
}

pub fn matrix(queens: &mut Vec<i32>, n: i32) -> Vec<String> {
    let mut arr = vec![];
    for i in queens {
        let mut char_vector = vec!['.'; n as usize];
        char_vector[*i as usize] = 'Q';
        let str: String = char_vector.into_iter().collect();
        arr.push(str);
    }

    arr
}