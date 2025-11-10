impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        // Dynamic programming array where dp[i] = number of set bits in i
        let mut dp = vec![0; (n + 1) as usize];
        
        // 'offset' keeps track of the most recent power of two
        // e.g., when i crosses 1, 2, 4, 8... we update the offset
        let mut offset: usize = 1;

        for i in 1..=n as usize {
            // When i hits the next power of 2, update offset
            if offset * 2 == i {
                offset = i;
            }
            // Number of 1s in i = 1 + dp[i - offset]
            dp[i] = 1 + dp[i - offset];
        }

        dp
    }
}