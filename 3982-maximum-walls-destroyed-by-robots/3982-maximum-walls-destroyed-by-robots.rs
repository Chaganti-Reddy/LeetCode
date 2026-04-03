impl Solution {
    pub fn max_walls(robots: Vec<i32>, distance: Vec<i32>, walls: Vec<i32>) -> i32 {
        let mut bots: Vec<(i32, i32)> = robots.into_iter().zip(distance).collect();
        bots.sort_unstable();

        let mut walls = walls;
        walls.sort_unstable();

        let count = |lo: i32, hi: i32| -> i32 {
            if lo > hi { return 0; }
            let l = walls.partition_point(|&w| w < lo);
            let r = walls.partition_point(|&w| w <= hi);
            (r - l) as i32
        };

        // walls exactly on robot positions â always destroyed
        let res_walls: i32 = bots.iter()
            .map(|&(pos, _)| count(pos, pos))
            .sum();

        let (first_pos, first_dist) = bots[0];
        // dp[0] = robot fires left, dp[1] = robot fires right
        let mut dp = [
            count(first_pos - first_dist, first_pos - 1), 
            0,                                            
        ];

        for i in 1..bots.len() {
            let (curr_pos, curr_dist) = bots[i];
            let (prev_pos, prev_dist) = bots[i - 1];

            let gap_lo = prev_pos + 1;
            let gap_hi = curr_pos - 1;

            let gap_walls   = count(gap_lo, gap_hi);
            let left_range  = count((curr_pos - curr_dist).max(gap_lo), gap_hi);
            let right_range = count(gap_lo, (prev_pos + prev_dist).min(gap_hi));

            // do their ranges overlap inside the gap?
            let overlap_lo = (curr_pos - curr_dist).max(gap_lo);
            let overlap_hi = (prev_pos + prev_dist).min(gap_hi);
            let extra = if overlap_lo <= overlap_hi { gap_walls } else { left_range + right_range };

            let new_dp = [
                (dp[0] + left_range).max(dp[1] + extra),
                dp[0].max(dp[1] + right_range),
            ];

            dp = new_dp;
        }

        // last robot fires right freely with no blocker
        let (last_pos, last_dist) = *bots.last().unwrap();
        let last_right = count(last_pos + 1, last_pos + last_dist);

        res_walls + dp[0].max(dp[1] + last_right)
    }
}