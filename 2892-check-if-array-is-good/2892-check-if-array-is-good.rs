impl Solution {
    pub fn is_good(mut nums: Vec<i32>) -> bool {
        let n = nums.len() as i32 - 1;

        for i in 0..nums.len() {
            let val = nums[i].abs();
            if val < 1 || val > n {
                return false;
            }
            let idx = (val - 1) as usize;
            if val == n {
                continue;
            }
            if nums[idx] < 0 {
                return false;
            }

            nums[idx] *= -1;
        }

        let mut count_n = 0;

        for &x in nums.iter() {
            if x.abs() == n {
                count_n += 1;
            }
        }

        count_n == 2
    }
}