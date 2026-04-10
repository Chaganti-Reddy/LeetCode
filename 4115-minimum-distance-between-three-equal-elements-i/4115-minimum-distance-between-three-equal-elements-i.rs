use std::collections::HashMap;

impl Solution {
    pub fn minimum_distance(nums: Vec<i32>) -> i32 {
        let mut seen: HashMap<i32, (usize, usize)> = HashMap::new();
        let mut min_dist = i32::MAX;

        for (k, &val) in nums.iter().enumerate() {
            match seen.get(&val) {
                None => { seen.insert(val, (usize::MAX, k)); }
                Some(&(usize::MAX, j)) => { seen.insert(val, (j, k)); }
                Some(&(i, j)) => {
                    min_dist = min_dist.min(2 * (k - i) as i32);
                    seen.insert(val, (j, k));
                }
            }
        }

        if min_dist == i32::MAX { -1 } else { min_dist }
    }
}