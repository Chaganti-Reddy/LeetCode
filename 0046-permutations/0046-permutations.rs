impl Solution {
    fn dfs(nums: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>, start: usize) {
        if start == nums.len() {
            ans.push(nums.clone());
            return;
        }

        for i in start..nums.len() {
            nums.swap(start, i);
            Self::dfs(nums, ans, start + 1);
            nums.swap(start, i);
        }
    }

    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans = Vec::new();
        let mut nums = nums;
        Self::dfs(&mut nums, &mut ans, 0);
        ans
    }
}
