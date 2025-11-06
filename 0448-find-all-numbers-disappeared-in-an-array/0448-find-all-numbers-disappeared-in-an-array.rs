impl Solution {
    pub fn find_disappeared_numbers(mut nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut ans = Vec::new();

        for i in 0..n {
            let idx = nums[i].abs() as usize - 1;
            nums[idx] = -nums[idx].abs();
        }

        for i in 0..n {
            if nums[i] > 0 {
                ans.push(i as i32 + 1);
            }
        }

        ans
    }
}
