//! Sudoku Solver [leetcode: sudoku_solver](https://leetcode.com/problems/sudoku-solver/)
//!
//! Write a program to solve a Sudoku puzzle by filling the empty cells.
//!
//! A sudoku solution must satisfy **all of the following rules**:
//! 1. Each of the digits `1-9` must occur exactly once in each row.
//! 2. Each of the digits `1-9` must occur exactly once in each column.
//! 3. Each of the the digits `1-9` must occur exactly once in each of the 9 `3x3` sub-boxes of the grid.
//! 4. Empty cells are indicated by the character `'.'`.
//!
//! <div>
//! <img src="https://upload.wikimedia.org/wikipedia/commons/thumb/f/ff/Sudoku-by-L2G-20050714.svg/250px-Sudoku-by-L2G-20050714.svg.png" style="height:250px; width:250px">
//! </div>
//!
//! A sudoku puzzle...
//! <div>
//! <img src="https://upload.wikimedia.org/wikipedia/commons/thumb/3/31/Sudoku-by-L2G-20050714_solution.svg/250px-Sudoku-by-L2G-20050714_solution.svg.png" style="height:250px; width:250px">
//! </div>
//!
//! ...and its solution numbers marked in red.
//!
//! **Note**:
//!
//! * The given board contain only digits `1-9` and the character `'.'`.
//! * You may assume that the given Sudoku puzzle will have a single unique solution.
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
/// * Runtime: 12 ms
/// * Memory: 2.4 MB
///
/// ```rust
/// impl Solution {
///     pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
///         if board.is_empty() { return; }
///         Self::solve(board);
///     }
///
///     pub fn solve(board: &mut Vec<Vec<char>>) -> bool {
///         for row in 0..board.len() {
///             for col in 0..board[row].len() {
///                 if board[row][col] != '.' { continue; }
///
///                 for c in "123456789".chars().collect::<Vec<_>>() {
///                     if !Self::is_valid(&board, row, col, c) { continue; }
///
///                     board[row][col] = c;
///
///                     if Self::solve(board) { return true; }
///
///                     board[row][col] = '.';
///                 }
///
///                 return false;
///             }
///         }
///         return true;
///     }
///
///     pub fn is_valid(board: &Vec<Vec<char>>, row: usize, col: usize, c: char) -> bool {
///         for i in 0..9 {
///             if board[row][i] != '.' && board[row][i] == c { return false; } // check row
///             if board[i][col] != '.' && board[i][col] == c { return false; } // check col
///             let block = board[(3 * (row as i32 / 3) + i as i32 / 3) as usize][(3 * (col as i32 / 3) + i as i32 % 3) as usize];
///             if  block != '.' && block == c { return false; } // check 3 * 3 block;
///         }
///         true
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
/// * Memory: 2.5 MB
///
/// ```rust
/// use std::char;
///
/// impl Solution {
///     pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
///         if board.is_empty() { return; }
///
///         // 快速检索第i, j列, 第k宫的数字是否被占用
///         let mut lines   = [[false; 9]; 9];
///         let mut columns = [[false; 9]; 9];
///         let mut ceils   = [[false; 9]; 9];
///         let mut origins = [[false; 9]; 9];
///
///         // initialize lines columns ceils and origins
///         for row in 0..9 {
///             for col in 0..9 {
///                 let num = match board[row][col].to_digit(10) {
///                     Some(n) => (n - 1) as usize,
///                     None => continue,
///                 };
///
///                 lines[row][num]   = true;
///                 columns[col][num] = true;
///                 origins[row][col] = true;
///                 ceils[Self::ceil_pos((row, col))][num] = true;
///             }
///         }
///
///         Self::solve(board, (0, 0), &mut lines, &mut columns, &mut ceils, &mut origins);
///     }
///
///     pub fn solve(board: &mut Vec<Vec<char>>,
///                 (row, col): (usize, usize),
///                 lines:   &mut[[bool; 9]; 9],
///                 columns: &mut[[bool; 9]; 9],
///                 ceils:   &mut[[bool; 9]; 9],
///                 origins: &mut[[bool; 9]; 9]) -> bool {
///
///         if row >= 9 { return true; }
///         let next_pos = (row + (col + 1) / 9, (col + 1) % 9);
///         if origins[row][col] { return Self::solve(board, next_pos, lines, columns, ceils, origins); }
///
///         let mut flag = false;
///         for num in 0..9 {
///             let ceil = Self::ceil_pos((row, col));
///             if lines[row][num] || columns[col][num] || ceils[ceil][num] { continue; }
///
///             // 数字num + 1没用过
///             lines[row][num]   = true;
///             columns[col][num] = true;
///             ceils[ceil][num]  = true;
///
///             board[row][col] = char::from_digit(num as u32 + 1, 10).unwrap();
///
///             flag |= Self::solve(board, next_pos, lines, columns, ceils, origins);
///
///             if flag { break; } // 已填数 OK
///             lines[row][num]   = false;
///             columns[col][num] = false;
///             ceils[ceil][num]  = false;
///         }
///
///         flag
///     }
///
///     // 求出pos属于第几个ceil
///     pub fn ceil_pos(pos: (usize, usize)) -> usize {
///         (pos.0 / 3) * 3 + pos.1 / 3
///     }
/// }
///

pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
    if board.is_empty() { return; }
    solve(board);
}

pub fn solve(board: &mut Vec<Vec<char>>) -> bool {
    for row in 0..board.len() {
        for col in 0..board[row].len() {
            if board[row][col] != '.' { continue; }

            for c in "123456789".chars().collect::<Vec<_>>() {
                if !is_valid(&board, row, col, c) { continue; }

                board[row][col] = c;

                if solve(board) { return true; }

                board[row][col] = '.';
            }

            return false;
        }
    }
    return true;
}

pub fn is_valid(board: &Vec<Vec<char>>, row: usize, col: usize, c: char) -> bool {
    for i in 0..9 {
        if board[row][i] != '.' && board[row][i] == c { return false; } // check row
        if board[i][col] != '.' && board[i][col] == c { return false; } // check col
        let block = board[(3 * (row as i32 / 3) + i as i32 / 3) as usize][(3 * (col as i32 / 3) + i as i32 % 3) as usize];
        if  block != '.' && block == c { return false; } // check 3 * 3 block;
    }
    true
}
