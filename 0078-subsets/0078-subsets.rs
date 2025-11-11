impl Solution {
    fn backtrack(nums: &Vec<i32>, path: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>, i: usize) {
        if i == nums.len() {
            ans.push(path.clone());
            return;
        }

        path.push(nums[i]);
        Self::backtrack(nums, path, ans, i + 1);
        path.pop();

        Self::backtrack(nums, path, ans, i + 1);
    }

    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans = Vec::new();
        let mut path = Vec::new();
        Self::backtrack(&nums, &mut path, &mut ans, 0);
        ans
    }
}
