use std::collections::VecDeque;

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut queue = VecDeque::new();
        let mut result: Vec<i32> = Vec::new();

        for (i, n) in nums.iter().enumerate() {
            while !queue.is_empty() && nums[*queue.back().unwrap()] < *n {
                queue.pop_back();
            }
            queue.push_back(i);
            if *queue.front().unwrap() + k as usize == i {
                queue.pop_front();
            }

            if i >= k as usize - 1{
                result.push(nums[*queue.front().unwrap()]);
            }
        }
        result
    }
}