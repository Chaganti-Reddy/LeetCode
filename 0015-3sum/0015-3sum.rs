impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort_unstable();
        let mut ans = Vec::new();

        for i in 0..nums.len().saturating_sub(2) {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }

            let (mut j, mut k) = (i + 1, nums.len() - 1);
            let target = -nums[i];

            while j < k {
                let sum = nums[j] + nums[k];
                if sum > target {
                    k -= 1;
                } else if sum < target {
                    j += 1;
                } else {
                    ans.push(vec![nums[i], nums[j], nums[k]]);
                    j += 1;
                    k -= 1;

                    while j < k && nums[j] == nums[j - 1] {
                        j += 1;
                    }
                    while j < k && nums[k] == nums[k + 1] {
                        k -= 1;
                    }
                }
            }
        }
        ans
    }
}
