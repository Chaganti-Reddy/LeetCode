impl Solution {
    fn reverse(mut n: i32) -> i32 {
        let mut ans = 0;

        while n > 0 {
            let digit = n % 10;
            n /= 10;

            if ans > i32::MAX / 10 {
                return 0;
            }

            ans = ans * 10 + digit;
        }

        ans
    }

    pub fn mirror_distance(n: i32) -> i32 {
        (n - Self::reverse(n)).abs()
    }
}