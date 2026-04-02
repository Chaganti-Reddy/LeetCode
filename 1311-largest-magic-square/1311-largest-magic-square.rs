impl Solution {
    pub fn largest_magic_square(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();

        let mut row_sum = vec![vec![0; n + 1]; m];
        for r in 0..m {
            for c in 0..n {
                row_sum[r][c + 1] = row_sum[r][c] + grid[r][c];
            }
        }

        let mut col_sum = vec![vec![0; m + 1]; n];
        for c in 0..n {
            for r in 0..m {
                col_sum[c][r + 1] = col_sum[c][r] + grid[r][c];
            }
        }

        for k in (2..=m.min(n)).rev() {
            for r in 0..=(m - k) {
                for c in 0..=(n - k) {
                    if Self::is_magic(k, r, c, &grid, &row_sum, &col_sum) {
                        return k as i32;
                    }
                }
            }
        }

        1
    }

    fn is_magic(k: usize, r: usize, c: usize, 
                grid: &Vec<Vec<i32>>, 
                row_sum: &Vec<Vec<i32>>, 
                col_sum: &Vec<Vec<i32>>) -> bool {
        
        let target = row_sum[r][c + k] - row_sum[r][c];

        for i in (r + 1)..(r + k) {
            if row_sum[i][c + k] - row_sum[i][c] != target {
                return false;
            }
        }

        for j in c..(c + k) {
            if col_sum[j][r + k] - col_sum[j][r] != target {
                return false;
            }
        }

        let mut diag1 = 0;
        for i in 0..k {
            diag1 += grid[r + i][c + i];
        }
        if diag1 != target {
            return false;
        }

        let mut diag2 = 0;
        for i in 0..k {
            diag2 += grid[r + i][c + k - 1 - i];
        }
        if diag2 != target {
            return false;
        }

        true
    }
}