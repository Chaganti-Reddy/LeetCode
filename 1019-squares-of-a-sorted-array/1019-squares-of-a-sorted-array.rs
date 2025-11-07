impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let (mut i, mut j, mut pos) = (0, nums.len() as usize - 1, nums.len() as usize -1);
        let mut ans = vec![0; nums.len() as usize]; 

        while i <= j{
            if nums[i]*nums[i] < nums[j]*nums[j] {
                ans[pos] = nums[j]*nums[j];
                j -= 1;
            } else {
                ans[pos] = nums[i]*nums[i];
                i += 1;
            }
            pos -= 1;
        }
        ans
    }
}