//! Number of Islands [leetcode: number_of_islands](https://leetcode.com/problems/number-of-islands/)
//!
//! Given a 2d grid map of `'1'`s (land) and `'0'`s (water), count the number of islands. An island is surrounded by water and is formed by connecting adjacent lands horizontally or vertically. You may assume all four edges of the grid are all surrounded by water.
//!
//! ***Example1:***
//!
//! ```
//! Input:
//! 11110
//! 11010
//! 11000
//! 00000
//!
//! Output: 1
//! ```
//!
//! ***Example2:***
//!
//! ```
//! Input:
//! 11000
//! 11000
//! 00100
//! 00011
//!
//! Output: 3
//! ```
//!

/// # Solutions
///
/// # Approach 1: Union Find
///
/// * Time complexity: O(m*n)
///
/// * Space complexity: O(m*n)
///
/// * Runtime: 4 ms
/// * Memory: 4.6 MB
///
/// ```rust
/// struct UnionFind {
///     parents: Vec<i32>,
///     count: i32,
///     rank: Vec<i32>,
/// }
///
/// impl UnionFind {
///     fn new(grid: &Vec<Vec<char>>) -> Self {
///         let m = grid.len();
///         let n = grid[0].len();
///         let mut count = 0;
///         let mut parents = vec![-1; m * n];
///         let rank = vec![0; m * n];
///         for i in 0..m {
///             for j in 0..n {
///                 if grid[i][j] == '1' {
///                     parents[i * n + j] = (i * n + j) as i32;
///                     count += 1;
///                 }
///             }
///         }
///         UnionFind { parents: parents, count: count, rank: rank }
///     }
///
///     fn find(&mut self, i: i32) -> i32 {
///         if self.parents[i as usize] != i {
///             self.parents[i as usize] = self.find(self.parents[i as usize])
///         };
///         self.parents[i as usize]
///     }
///
///     fn union(&mut self, x: i32, y: i32) -> i32 {
///         let x_parent = self.find(x);
///         let y_parent = self.find(y);
///
///         if x_parent == y_parent { return self.count; }
///
///         if self.rank[x_parent as usize] < self.rank[y_parent as usize] {
///             self.parents[x_parent as usize] = y_parent;
///         } else if self.rank[x_parent as usize] > self.rank[y_parent as usize] {
///             self.parents[y_parent as usize] = x_parent;
///         } else {
///             self.parents[y_parent as usize] = x_parent;
///             self.rank[y_parent as usize] += 1;
///         }
///         self.count -= 1;
///
///         self.count
///     }
/// }
///
/// impl Solution {
///     pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
///         if grid.is_empty() { return 0; }
///
///         let directions: Vec<(i32, i32)> = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
///         let (m, n) = (grid.len(), grid[0].len());
///         let mut uf = UnionFind::new(&grid);
///
///         for i in 0..m {
///             for j in 0..n {
///                 if grid[i][j] == '0' { continue; }
///
///                 for d in &directions {
///                     let (dr, dc) = (i as i32 + d.0, j as i32 + d.1);
///                     if dr >= 0 &&
///                        dc >= 0 &&
///                        dr < m as i32 &&
///                        dc < n as i32 &&
///                        grid[dr as usize][dc as usize] == '1' {
///                         uf.union((i * n + j) as i32, dr * n as i32 + dc);
///                     }
///                 }
///             }
///         }
///         uf.count
///
///     }
/// }
/// ```
///
/// # Approach 2: DFS
///
/// * Time complexity: O(m*n)
///
/// * Space complexity: O(m*n)
///
/// * Runtime: 16ms
/// * Memory: 4.7MB
///
/// ```rust
/// use std::collections::HashSet;
///
/// impl Solution {
///     pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
///         if grid.is_empty() { return 0; }
///
///         let mut visited: HashSet<(usize, usize)> = HashSet::new();
///         let mut result = 0;
///         let m = grid.len();
///         let n = grid[0].len();
///
///         for i in 0..m {
///             for j in 0..n {
///                 if !visited.contains(&(i, j)) && grid[i][j] == '1' {
///                     Self::dfs(&grid, &mut visited, i, j);
///                     result += 1;
///                 }
///             }
///         }
///
///         result
///     }
///
///     const DIRECTIONS: [[i32; 2]; 4] = [[1, 0], [-1, 0], [0, 1], [0, -1]];
///     pub fn dfs(grid: &Vec<Vec<char>>, visited: &mut HashSet<(usize, usize)>, m: usize, n: usize) {
///         if m >= grid.len() || n >= grid[0].len() || grid[m][n] == '0' || visited.contains(&(m, n)) { return; }
///
///         visited.insert((m, n));
///         for dis in Self::DIRECTIONS.iter() {
///             let dr = (dis[0] + m as i32) as usize;
///             let dc = (dis[1] + n as i32) as usize;
///             Self::dfs(grid, visited, dr, dc);
///         }
///     }
/// }
/// ```
///

pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
    if grid.is_empty() { return 0; }

    let directions: Vec<(i32, i32)> = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
    let (m, n) = (grid.len(), grid[0].len());
    let mut uf = UnionFind::new(&grid);

    for i in 0..m {
        for j in 0..n {
            if grid[i][j] == '0' { continue; }

            for d in &directions {
                let (dr, dc) = (i as i32 + d.0, j as i32 + d.1);
                if dr >= 0 &&
                   dc >= 0 &&
                   dr < m as i32 &&
                   dc < n as i32 &&
                   grid[dr as usize][dc as usize] == '1' {
                    uf.union((i * n + j) as i32, dr * n as i32 + dc);
                }
            }
        }
    }
    uf.count
}

#[derive(Debug, Default)]
struct UnionFind {
    parents: Vec<i32>,
    count: i32,
    rank: Vec<i32>,
}

impl UnionFind {
    fn new(grid: &Vec<Vec<char>>) -> Self {
        let m = grid.len();
        let n = grid[0].len();
        let mut count = 0;
        let mut parents = vec![-1; m * n];
        let rank = vec![0; m * n];
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == '1' {
                    parents[i * n + j] = (i * n + j) as i32;
                    count += 1;
                }
            }
        }
        UnionFind { parents: parents, count: count, rank: rank }
    }

    fn find(&mut self, i: i32) -> i32 {
        if self.parents[i as usize] != i {
            self.parents[i as usize] = self.find(self.parents[i as usize])
        };
        self.parents[i as usize]
    }

    fn union(&mut self, x: i32, y: i32) -> i32 {
        let x_parent = self.find(x);
        let y_parent = self.find(y);

        if x_parent == y_parent { return self.count; }

        if self.rank[x_parent as usize] < self.rank[y_parent as usize] {
            self.parents[x_parent as usize] = y_parent;
        } else if self.rank[x_parent as usize] > self.rank[y_parent as usize] {
            self.parents[y_parent as usize] = x_parent;
        } else {
            self.parents[y_parent as usize] = x_parent;
            self.rank[y_parent as usize] += 1;
        }
        self.count -= 1;

        self.count
    }
}
