impl Solution {
    pub fn running_sum(mut nums: Vec<i32>) -> Vec<i32> {
        for i in 1..nums.iter().len(){
            nums[i] += nums[i-1];
        }
        nums
    }
}