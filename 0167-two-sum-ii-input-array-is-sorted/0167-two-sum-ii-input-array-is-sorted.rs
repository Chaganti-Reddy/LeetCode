impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let (mut i, mut j) = (0, numbers.len() - 1);
        while i < j {
            let sum = numbers[i] + numbers[j];
            if sum < target {
                i += 1;
            } else if sum > target {
                j -= 1;
            } else {
                return vec![i as i32 + 1, j as i32 + 1];
            }
        }
        vec![]
    }
}
