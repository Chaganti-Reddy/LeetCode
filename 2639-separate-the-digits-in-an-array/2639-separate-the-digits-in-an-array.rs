impl Solution {
    pub fn separate_digits(nums: Vec<i32>) -> Vec<i32> {
        let mut ans = Vec::new();

        for num in nums {
            let mut stack = Vec::new();
            let mut n = num;

            if n == 0 {
                ans.push(0);
                continue;
            }

            while n > 0 {
                stack.push(n % 10);
                n /= 10;
            }

            while let Some(d) = stack.pop() {
                ans.push(d);
            }
        }

        ans
    }
}