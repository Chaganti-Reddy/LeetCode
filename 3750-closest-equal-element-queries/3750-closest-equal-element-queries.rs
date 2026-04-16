use std::collections::HashMap;

impl Solution {
    pub fn solve_queries(nums: Vec<i32>, queries: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut next = vec![i32::MAX; n];
        let mut prev = vec![i32::MAX; n];

        let mut freq = HashMap::new();
        for &num in &nums {
            *freq.entry(num).or_insert(0) += 1;
        }

        let mut last_seen = HashMap::new();

        for i in 0..2 * n {
            let idx = i % n;
            if let Some(&last) = last_seen.get(&nums[idx]) {
                next[last % n] = (i - last) as i32;
            }
            last_seen.insert(nums[idx], i);
        }

        last_seen.clear();

        for i in (0..2 * n).rev() {
            let idx = i % n;
            if let Some(&last) = last_seen.get(&nums[idx]) {
                prev[last % n] = (last - i) as i32;
            }
            last_seen.insert(nums[idx], i);
        }

        queries
            .into_iter()
            .map(|q| {
                let i = q as usize;

                if freq[&nums[i]] == 1 {
                    return -1;
                }

                next[i].min(prev[i])
            })
            .collect()
    }
}