impl Solution {
    pub fn rotate_the_box(mut box_grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
        let m = box_grid.len();
        let n = box_grid[0].len();

        for i in 0..m {
            let mut empty = n as i32 - 1;

            for j in (0..n).rev() {
                if box_grid[i][j] == '*' {
                    empty = j as i32 - 1;
                } else if box_grid[i][j] == '#' {
                    let e = empty as usize;
                    box_grid[i][j] = '.';
                    box_grid[i][e] = '#';
                    empty -= 1;
                }
            }
        }

        let mut res = vec![vec!['.'; m]; n];

        for i in 0..m {
            for j in 0..n {
                res[j][m - 1 - i] = box_grid[i][j];
            }
        }

        res
    }
}