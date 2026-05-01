impl Solution {
    pub fn max_rotate_function(nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;
        
        let sum: i32 = nums.iter().sum();
        
        let mut f: i32 = nums.iter()
            .enumerate()
            .map(|(i, &v)| i as i32 * v)
            .sum();
        
        let mut ans = f;
        
        for k in 1..nums.len() {
            f = f + sum - n * nums[nums.len() - k];
            ans = ans.max(f);
        }
        
        ans
    }
}