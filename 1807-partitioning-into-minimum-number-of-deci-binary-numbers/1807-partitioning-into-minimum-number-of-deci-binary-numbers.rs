impl Solution {
    pub fn min_partitions(n: String) -> i32 {
        n.bytes()
            .max()
            .map(|b| (b - b'0') as i32)
            .unwrap_or(0)
    }
}