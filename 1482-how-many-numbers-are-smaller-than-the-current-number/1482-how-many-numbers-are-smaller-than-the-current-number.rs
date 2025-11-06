impl Solution {
    pub fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
        let mut count = vec![0; 101];

        for &num in &nums {
            count[num as usize] += 1;
        }

        for i in 1..101 {
            count[i] += count[i - 1];
        }

        nums.iter()
            .map(|&num| if num == 0 { 0 } else { count[(num - 1) as usize] })
            .collect()
    }
}
