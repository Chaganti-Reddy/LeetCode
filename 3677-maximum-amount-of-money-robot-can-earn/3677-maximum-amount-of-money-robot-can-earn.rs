impl Solution {
    pub fn maximum_amount(coins: Vec<Vec<i32>>) -> i32 {
        let m = coins.len();
        let n = coins[0].len();
        const NEG_INF: i64 = -1e15 as i64;
        let mut dp = vec![[NEG_INF; 3]; n];

        for i in 0..m {
            for j in 0..n {
                let val = coins[i][j] as i64;
                let mut current = [NEG_INF; 3];

                if i == 0 && j == 0 {
                    current[0] = val;
                    current[1] = if val < 0 { 0 } else { val };
                    current[2] = if val < 0 { 0 } else { val };
                } else {                    
                    for k in 0..3 {
                        let mut best_prev = NEG_INF;
                        if i > 0 { best_prev = best_prev.max(dp[j][k]); }
                        if j > 0 { best_prev = best_prev.max(dp[j-1][k]); }

                        current[k] = best_prev + val;

                        if val < 0 && k > 0 {
                            let mut best_prev_for_skip = NEG_INF;
                            if i > 0 { best_prev_for_skip = best_prev_for_skip.max(dp[j][k-1]); }
                            if j > 0 { best_prev_for_skip = best_prev_for_skip.max(dp[j-1][k-1]); }
                            current[k] = current[k].max(best_prev_for_skip);
                        }
                    }
                }
                dp[j] = current;
            }
        }

        dp[n - 1].iter().max().copied().unwrap() as i32
    }
}