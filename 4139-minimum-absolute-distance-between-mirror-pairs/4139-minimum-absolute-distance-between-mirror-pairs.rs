use std::collections::HashMap;

impl Solution {
    fn reverse_num(mut num: i32) -> i32 {
        let mut reversed = 0i32;
        while num > 0 {
            reversed = reversed * 10 + num % 10;
            num /= 10;
        }
        reversed
    }

    pub fn min_mirror_pair_distance(nums: Vec<i32>) -> i32 {
        let mut index_map: HashMap<i32, Vec<usize>> = HashMap::new();
        let mut min_dist = i32::MAX;
        
        for (j, &num) in nums.iter().enumerate() {
            let rev = Self::reverse_num(num);
            if let Some(indices) = index_map.get(&num) {
                let closest = indices.last().unwrap();
                min_dist = min_dist.min((j - closest) as i32);
            }
            index_map.entry(rev).or_default().push(j);
        }

        if min_dist == i32::MAX { -1 } else { min_dist }
    }
}