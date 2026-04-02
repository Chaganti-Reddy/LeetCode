impl Solution {
    pub fn num_sub(s: String) -> i32 {
        let m = 1_000_000_007;
        let mut ans = 0;
        let mut streak = 0;

        for b in s.bytes() {
            if b == b'1' {
                streak += 1;
                ans = (ans + streak) % m;
            } else {
                streak = 0;
            }
        }

        ans
    }
}