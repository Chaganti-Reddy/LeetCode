impl Solution {
    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        if grid.is_empty() || grid[0].is_empty() {
            return 0;
        }

        let mut count = 0;
        let rows = grid.len();
        let cols = grid[0].len();

        for i in 0..rows {
            for j in 0..cols {
                if grid[i][j] == '1' {
                    count += 1;
                    Self::dfs(&mut grid, i as isize, j as isize);
                }
            }
        }
        count
    }

    fn dfs(grid: &mut Vec<Vec<char>>, i: isize, j: isize) {
        let rows = grid.len() as isize;
        let cols = grid[0].len() as isize;

        if i < 0 || j < 0 || i >= rows || j >= cols || grid[i as usize][j as usize] == '0' {
            return;
        }

        grid[i as usize][j as usize] = '0';

        Self::dfs(grid, i + 1, j);
        Self::dfs(grid, i - 1, j);
        Self::dfs(grid, i, j + 1);
        Self::dfs(grid, i, j - 1);
    }
}
