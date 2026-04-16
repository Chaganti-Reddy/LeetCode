use std::collections::HashMap;

impl Solution {
    pub fn solve_queries(mut nums: Vec<i32>,mut queries: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut num_indices = HashMap::<i32,(usize,usize)>::new();
        for i in 0..n{
            if let Some((first,prev)) = num_indices.get_mut(&nums[i]){
                nums[i] = (i-*prev).min(n-i+*first) as i32;
                nums[*first] = (nums[*first] as u32).min((*first+n-i) as u32) as i32;
                nums[*prev] = (nums[*prev] as u32).min((i-*prev) as u32) as i32;
                *prev = i;
            }else{
                num_indices.insert(nums[i],(i,i));
                nums[i] = -1;
            }
        }
        queries.into_iter().map(|q| nums[q as usize] ).collect::<Vec<_>>()
    }
}