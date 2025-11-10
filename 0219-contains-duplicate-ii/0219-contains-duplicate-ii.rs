use std::collections::HashSet;

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let mut window = HashSet::new();
        let k = k as usize;

        for i in 0..nums.len() {
            if window.contains(&nums[i]) {
                return true;
            }
            window.insert(nums[i]);
            if window.len() > k {
                window.remove(&nums[i - k]);
            }
        }

        false
    }
}
