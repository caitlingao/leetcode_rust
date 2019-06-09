//! Word Search [leetcode: word_search](https://leetcode.com/problems/word-search/)
//!
//! Given a 2D board and a word, find if the word exists in the grid.
//!
//! The word can be constructed from letters of sequentially adjacent cell, where "adjacent" cells are those horizontally or vertically neighboring. The same letter cell may not be used more than once.
//!
//! **Example 1:**
//! ```
//! board =
//! [
//!   ['A','B','C','E'],
//!   ['S','F','C','S'],
//!   ['A','D','E','E']
//! ]
//!
//! Given word = "ABCCED", return true.
//! Given word = "SEE", return true.
//! Given word = "ABCB", return false.
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
/// * Runtime: 8 ms
/// * Memory: 3.5 MB
///
/// ```rust
/// impl Solution {
///     const DIRS: [[i32; 2]; 4] = [[1, 0], [-1, 0], [0, 1], [0, -1]];
///
///     pub fn exist(mut board: Vec<Vec<char>>, word: String) -> bool {
///         if board.is_empty() || board[0].is_empty() { return false; }
///         if word.is_empty() { return true; }
///
///         let m = board.len();
///         let n = board[0].len();
///         let word: Vec<char> = word.chars().collect();
///         let mut used = vec![vec![false; n]; m];
///
///         for i in 0..m {
///             for j in 0..n {
///                 if Self::dfs(&board, &word, &mut used, i, j) { return true; }
///             }
///         }
///
///         false
///     }
///
///     pub fn dfs(board: &Vec<Vec<char>>,
///                word:  &[char],
///                used:  &mut Vec<Vec<bool>>,
///                i:     usize,
///                j:     usize) -> bool {
///         if i >= board.len() || j >= board[0].len() || used[i][j] { return false; }
///
///         if board[i][j] != word[0] { return false; }
///         if word.len() == 1 { return true; }
///
///         used[i][j] = true;
///         for d in Self::DIRS.iter() {
///             let ni = (i as i32 + d[0]) as usize;
///             let nj = (j as i32 + d[1]) as usize;
///             if Self::dfs(board, &word[1..], used, ni, nj) { return true; }
///         }
///         used[i][j] = false;
///         false
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
/// * Runtime: 8 ms
/// * Memory: 3.4 MB
///
/// ```rust
/// impl Solution {
///     const DIRS: [[i32; 2]; 4] = [[1, 0], [-1, 0], [0, 1], [0, -1]];
///
///     pub fn exist(mut board: Vec<Vec<char>>, word: String) -> bool {
///         let (height, width) = (board.len(), board[0].len());
///         if height == 0 || width == 0 || word.len() < 1 { return false }
///         let seq: Vec<char> = word.chars().collect();
///
///         for row in 0..height {
///             for col in 0..width {
///                 if board[row][col] == seq[0] && Self::dfs(&mut board, &seq, row, col, 0) {
///                     return true
///                 }
///             }
///         }
///         false
///     }
///
///     fn dfs(board: &mut Vec<Vec<char>>, seq: &[char], row: usize, col: usize, count: i32) -> bool {
///         if count == seq.len() as i32 { return true; }
///         if row >= board.len() || col >= board[0].len() || board[row][col] != seq[count as usize] { return false; }
///
///         let ch = board[row][col];
///         board[row][col] = '#';
///         for d in Self::DIRS.iter() {
///             let ni = (row as i32 + d[0]) as usize;
///             let nj = (col as i32 + d[1]) as usize;
///             if Self::dfs(board, seq, ni, nj, count+1) { return true; }
///         }
///         board[row][col] = ch;
///         false
///     }
/// }
/// ```
///
pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
    if board.is_empty() || board[0].is_empty() { return false; }
    if word.is_empty() { return true; }

    let m = board.len();
    let n = board[0].len();
    let word: Vec<char> = word.chars().collect();
    let mut used = vec![vec![false; n]; m];

    for i in 0..m {
        for j in 0..n {
            if dfs(&board, &word, &mut used, i, j) { return true; }
        }
    }

    false
}

const DIRS: [[i32; 2]; 4] = [[1, 0], [-1, 0], [0, 1], [0, -1]];

pub fn dfs(board: &Vec<Vec<char>>,
           word:  &[char],
           used:  &mut Vec<Vec<bool>>,
           i:     usize,
           j:     usize) -> bool {
    if i >= board.len() || j >= board[0].len() || used[i][j] { return false; }

    if board[i][j] != word[0] { return false; }
    if word.len() == 1 { return true; }

    used[i][j] = true;
    for d in DIRS.iter() {
        let ni = (i as i32 + d[0]) as usize;
        let nj = (j as i32 + d[1]) as usize;
        if dfs(board, &word[1..], used, ni, nj) { return true; }
    }
    used[i][j] = false;
    false
}
