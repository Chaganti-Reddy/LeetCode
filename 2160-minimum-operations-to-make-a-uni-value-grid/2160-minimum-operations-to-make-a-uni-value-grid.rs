impl Solution {
    pub fn min_operations(mut grid: Vec<Vec<i32>>, x: i32) -> i32 {
        let mut nums: Vec<i32> = grid.into_iter().flatten().collect();
        
        let r = nums[0] % x;
        if nums.iter().any(|&v| v % x != r) {
            return -1;
        }
        
        let k = nums.len() / 2;
        nums.select_nth_unstable(k);
        let median = nums[k];
        
        nums.into_iter()
            .map(|v| (v - median).abs() / x)
            .sum()
    }
}