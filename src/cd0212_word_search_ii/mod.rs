//! Word Search II [leetcode: word_search_ii](https://leetcode.com/problems/word-search-ii/)
//!
//! Given a 2D board and a list of words from the dictionary, find all words in the board.
//!
//! Each word must be constructed from letters of sequentially adjacent cell, where "adjacent" cells are those horizontally or vertically neighboring. The same letter cell may not be used more than once in a word.
//!
//! **Example:**
//!
//! ```
//! Input:
//! board = [
//!   ['o','a','a','n'],
//!   ['e','t','a','e'],
//!   ['i','h','k','r'],
//!   ['i','f','l','v']
//! ]
//! words = ["oath","pea","eat","rain"]
//!
//! Output: ["eat","oath"]
//! ```
//!
//! **Note:**
//!
//! 1. All inputs are consist of lowercase letters `a-z`.
//! 2. The values of `words` are distinct.
use std::collections::HashSet;

const DIRS: [[i32; 2]; 4] = [[1, 0], [-1, 0], [0, 1], [0, -1]];
/// # Solutions
///
/// # Approach : Trie and DFS
///
/// * Time complexity:
///
/// * Space complexity:
/// * **It doesn't work, I'm sorry**
///
/// ```rust
/// use std::collections::HashSet;
///
/// #[derive(Debug, Default)]
/// pub struct Trie {
///     is_end: bool,
///     nodes: [Option<Box<Trie>>; 26],
/// }
///
/// impl Trie {
///
///     fn new() -> Self {
///         Default::default()
///     }
///
///     fn insert(&mut self, word: String) {
///         let mut curr = self;
///         for i in word.chars().map(|char| (char as u8 - 'a' as u8) as usize) {
///             curr = curr.nodes[i].get_or_insert_with(|| Box::new(Trie::new()));
///         }
///         curr.is_end = true;
///     }
/// }
///
/// impl Solution {
///     const DIRS: [[i32; 2]; 4] = [[1, 0], [-1, 0], [0, 1], [0, -1]];
///
///     pub fn find_words(mut board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
///         let rows = board.len();
///         let cols = board[0].len();
///
///         if rows == 0 || cols == 0 || words.len() == 0 { return vec![]; }
///
///         let mut trie = Trie::new();
///         let mut result: HashSet<String> = HashSet::new();
///
///         for word in words { trie.insert(word); }
///
///         for row in 0..rows {
///             for col in 0..cols {
///                 let mut s = String::from("");
///                 Self::_dfs(&mut board, &mut result, &mut s, row, col, &trie);
///             }
///         }
///
///         result.into_iter().collect::<Vec<String>>()
///     }
///
///     pub fn _dfs(board:  &mut Vec<Vec<char>>,
///                 result: &mut HashSet<String>,
///                 s:      &mut String,
///                 row:    usize,
///                 col:    usize,
///                 trie:   &Trie) {
///         let mut curr = trie;
///         if let Some(node) = trie.nodes[(board[row][col] as u8 - 'a' as u8) as usize].as_ref() {
///             curr = node;
///             s.push(board[row][col]);
///             if curr.is_end { result.insert(s.clone()); }
///             for d in Self::DIRS.iter() {
///                 let ni = (row as i32 + d[0]) as usize;
///                 let nj = (col as i32 + d[1]) as usize;
///                 if ni >= board.len() || nj >= board[0].len() || board[ni][nj] == '#' { continue; }
///
///                 let ch = board[row][col];
///                 board[row][col] = '#';
///                 Self::_dfs(board, result, s, ni, nj, curr);
///                 board[row][col] = ch;
///             }
///         }
///     }
/// }
/// ```
///
pub fn find_words(mut board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
    let rows = board.len();
    let cols = board[0].len();

    if rows == 0 || cols == 0 || words.len() == 0 { return vec![]; }

    let mut trie = Trie::new();
    let mut result: HashSet<String> = HashSet::new();

    for word in words { trie.insert(word); }

    for row in 0..rows {
        for col in 0..cols {
            let mut s = String::from("");
            _dfs(&mut board, &mut result, &mut s, row, col, &trie);
        }
    }

    result.into_iter().collect::<Vec<String>>()
}

pub fn _dfs(board:  &mut Vec<Vec<char>>,
            result: &mut HashSet<String>,
            s:      &mut String,
            row:    usize,
            col:    usize,
            trie:   &Trie) {
    if let Some(node) = trie.nodes[(board[row][col] as u8 - 'a' as u8) as usize].as_ref() {
        let curr = node;
        s.push(board[row][col]);
        if curr.is_end { result.insert(s.clone()); }
        for d in DIRS.iter() {
            let ni = (row as i32 + d[0]) as usize;
            let nj = (col as i32 + d[1]) as usize;
            if ni >= board.len() || nj >= board[0].len() || board[ni][nj] == '#' { continue; }

            let ch = board[row][col];
            board[row][col] = '#';
            _dfs(board, result, s, ni, nj, curr);
            board[row][col] = ch;
        }
    }
}

#[derive(Debug, Default)]
pub struct Trie {
    is_end: bool,
    nodes: [Option<Box<Trie>>; 26],
}

impl Trie {

    fn new() -> Self {
        Default::default()
    }

    fn insert(&mut self, word: String) {
        let mut curr = self;
        for i in word.chars().map(|char| (char as u8 - 'a' as u8) as usize) {
            curr = curr.nodes[i].get_or_insert_with(|| Box::new(Trie::new()));
        }
        curr.is_end = true;
    }
}
