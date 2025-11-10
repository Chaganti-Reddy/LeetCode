impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let mut dp = vec![amount + 1; (amount + 1) as usize];
        dp[0] = 0;

        for i in 1..=amount {
            for &coin in &coins {
                if i - coin >= 0 {
                    dp[i as usize] = dp[i as usize].min(1 + dp[(i - coin) as usize]);
                }
            }
        }

        if dp[amount as usize] > amount {
            -1
        } else {
            dp[amount as usize]
        }
    }
}
