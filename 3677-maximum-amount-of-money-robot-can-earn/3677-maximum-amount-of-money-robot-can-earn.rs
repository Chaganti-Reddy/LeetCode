impl Solution {
    pub fn maximum_amount(coins: Vec<Vec<i32>>) -> i32 {
        let m = coins.len();
        let n = coins[0].len();
        let neg_inf = -1e15 as i64;
        let mut prev_row = vec![vec![neg_inf; 3]; n];

        for i in 0..m {
            let mut curr_row = vec![vec![neg_inf; 3]; n];
            
            for j in 0..n {
                let val = coins[i][j] as i64;

                if i == 0 && j == 0 {
                    curr_row[0][0] = val;
                    if val < 0 { curr_row[0][1] = 0; }
                } else {
                    for k in 0..3 {
                        let mut prev_same_k = neg_inf;
                        if i > 0 { prev_same_k = prev_same_k.max(prev_row[j][k]); }
                        if j > 0 { prev_same_k = prev_same_k.max(curr_row[j-1][k]); }
                        
                        if prev_same_k != neg_inf {
                            curr_row[j][k] = curr_row[j][k].max(prev_same_k + val);
                        }

                        if val < 0 && k > 0 {
                            let mut prev_less_k = neg_inf;
                            if i > 0 { prev_less_k = prev_less_k.max(prev_row[j][k-1]); }
                            if j > 0 { prev_less_k = prev_less_k.max(curr_row[j-1][k-1]); }
                            
                            if prev_less_k != neg_inf {
                                curr_row[j][k] = curr_row[j][k].max(prev_less_k);
                            }
                        }
                    }
                }
            }
            prev_row = curr_row;
        }

        let res = prev_row[n-1][0].max(prev_row[n-1][1]).max(prev_row[n-1][2]);
        res as i32
    }
}