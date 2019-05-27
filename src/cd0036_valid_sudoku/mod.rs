//! Valid Sudoku [leetcode: valid_sudoku](https://leetcode.com/problems/valid-sudoku/)
//!
//!
//! Determine if a 9x9 Sudoku board is valid. Only the filled cells need to be validated **all of the following rules**:
//! 1. Each row must contain the digits `1-9` without repetition.
//! 2. Each column must contain the digits `1-9` without repetition.
//! 3. Each of the 9 `3x3` sub-boxes of the grid must contain the digits `1-9` without repetition.
//!
//! <div>
//! <img src="https://upload.wikimedia.org/wikipedia/commons/thumb/f/ff/Sudoku-by-L2G-20050714.svg/250px-Sudoku-by-L2G-20050714.svg.png" style="height:250px; width:250px">
//! </div>
//!
//! The Sudoku board could be partially filled, where empty cells are filled with the character `'.'`.
//!
//! **Example 1:**
//! ```
//! Input:
//! [
//!   ["5","3",".",".","7",".",".",".","."],
//!   ["6",".",".","1","9","5",".",".","."],
//!   [".","9","8",".",".",".",".","6","."],
//!   ["8",".",".",".","6",".",".",".","3"],
//!   ["4",".",".","8",".","3",".",".","1"],
//!   ["7",".",".",".","2",".",".",".","6"],
//!   [".","6",".",".",".",".","2","8","."],
//!   [".",".",".","4","1","9",".",".","5"],
//!   [".",".",".",".","8",".",".","7","9"]
//! ]
//! ```
//! Output: true
//!
//! ** Example 2:**
//! ```
//! Input:
//! [
//!   ["8","3",".",".","7",".",".",".","."],
//!   ["6",".",".","1","9","5",".",".","."],
//!   [".","9","8",".",".",".",".","6","."],
//!   ["8",".",".",".","6",".",".",".","3"],
//!   ["4",".",".","8",".","3",".",".","1"],
//!   ["7",".",".",".","2",".",".",".","6"],
//!   [".","6",".",".",".",".","2","8","."],
//!   [".",".",".","4","1","9",".",".","5"],
//!   [".",".",".",".","8",".",".","7","9"]
//! ]
//! Output: false
//! Explanation: Same as Example 1, except with the 5 in the top left corner being
//!     modified to 8. Since there are two 8's in the top left 3x3 sub-box, it is invalid.
//! ```
//!
//! **Note**:
//!
//! * A Sudoku board (partially filled) could be valid but is not necessarily solvable.
//! * Only the filled cells need to be validated according to the mentioned rules.
//! * The given board contain only digits `1-9` and the character `'.'`.
//! * The given board size is always `9x9`.
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
/// * Memory: 2.5 MB
///
/// ```rust
/// impl Solution {
///     pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
///         // 快速检索第i, j列, 第k宫的数字是否被占用
///         let mut line   = [[false; 9]; 9]; // j
///         let mut column = [[false; 9]; 9]; // i
///         let mut ceil   = [[false; 9]; 9]; // k
///         let mut origin = [[false; 9]; 9]; // 原始数字位置
///         // 初始化
///         for row in 0..9 {
///             for col in 0..9 {
///                 let num = match board[row][col].to_digit(10) {
///                     Some(n) => (n - 1) as usize,
///                     None    => continue
///                 };
///
///                 if line[row][num] || column[col][num] ||ceil[Self::ceil_k((row, col))][num] { return false; }
///                 line[row][num]   = true;
///                 column[col][num] = true;
///                 origin[row][col] = true;
///                 ceil[Self::ceil_k((row, col))][num] = true;
///             }
///         }
///         true
///     }
///
///     // 求出pos属于第几个ceil
///     fn ceil_k(pos: (usize, usize)) -> usize {
///         (pos.0 / 3) * 3 + pos.1 / 3
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
/// * Runtime: 5 ms
/// * Memory: 2.5 MB
///
/// ```rust
/// impl Solution {
///     pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
///         let mut table = vec![false; 9];
///
///         Solution::check_row(&board, &mut table) &&
///         Solution::check_col(&board, &mut table) &&
///         Solution::check_block(&board, &mut table)
///     }
///
///     pub fn check_row(board: &Vec<Vec<char>>, table: &mut Vec<bool>) -> bool {
///         // check row
///         for row in board.iter() {
///             for pos in 0..9 { table[pos] = false; }
///             for col in row {
///                 match col.to_digit(10) {
///                     Some(n) => {
///                         if table[(n-1) as usize] { return false; }
///                         table[(n-1) as usize] = true;
///                     },
///                     None => continue,
///                 }
///             }
///         }
///         true
///     }
///
///     pub fn check_col(board: &Vec<Vec<char>>, table: &mut Vec<bool>) -> bool {
///         // check col
///         for col in 0..9 {
///             for pos in 0..9 { table[pos] = false; }
///             for row in board.iter() {
///                 match row[col].to_digit(10) {
///                     Some(n) => {
///                         if table[(n-1) as usize] { return false; }
///                         table[(n-1) as usize] = true;
///                     },
///                     None => continue,
///                 }
///             }
///         }
///         true
///     }
///
///     pub fn check_block(board: &Vec<Vec<char>>, table: &mut Vec<bool>) -> bool {
///         // check 3 * 3 block
///         for i in 0..3 {
///             for j in 0..3 {
///                 for pos in 0..9 { table[pos] = false; }
///                 for row in 3 * i .. 3 * (i + 1) {
///                     for col in 3 * j .. 3 * (j + 1) {
///                         match board[row][col].to_digit(10) {
///                             Some(n) => {
///                                 if table[(n-1) as usize] { return false; }
///                                 table[(n-1) as usize] = true;
///                             },
///                             None => continue,
///                         }
///                     }
///                 }
///             }
///         }
///         true
///     }
/// }
///

pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    // 快速检索第i, j列, 第k宫的数字是否被占用
    let mut line   = [[false; 9]; 9]; // j
    let mut column = [[false; 9]; 9]; // i
    let mut ceil   = [[false; 9]; 9]; // k
    let mut origin = [[false; 9]; 9]; // 原始数字位置
    // 初始化
    for row in 0..9 {
        for col in 0..9 {
            let num = match board[row][col].to_digit(10) {
                Some(n) => (n - 1) as usize,
                None    => continue
            };

            if line[row][num] || column[col][num] ||ceil[ceil_k((row, col))][num] { return false; }
            line[row][num]   = true;
            column[col][num] = true;
            origin[row][col] = true;
            ceil[ceil_k((row, col))][num] = true;
        }
    }
    true
}

// 求出pos属于第几个ceil
fn ceil_k(pos: (usize, usize)) -> usize {
    (pos.0 / 3) * 3 + pos.1 / 3
}
