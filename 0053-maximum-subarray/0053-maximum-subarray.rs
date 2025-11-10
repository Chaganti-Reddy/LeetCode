impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let (mut max_sum, mut cur_sum) = (nums[0], nums[0]);

        for i in 1..nums.len() {
            cur_sum = nums[i].max(nums[i] + cur_sum);
            max_sum = max_sum.max(cur_sum); 
        }
        max_sum
    }
}