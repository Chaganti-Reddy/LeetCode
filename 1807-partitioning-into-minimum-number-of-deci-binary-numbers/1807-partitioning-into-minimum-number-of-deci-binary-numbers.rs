impl Solution {
    pub fn min_partitions(n: String) -> i32 {
        let mut m: u32 = 0;
        for c in n.chars() {    
            m = m.max(c.to_digit(10).unwrap());
        }
        m as i32
    }
}