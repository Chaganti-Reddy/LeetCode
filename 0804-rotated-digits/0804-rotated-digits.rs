impl Solution {
    pub fn rotated_digits(n: i32) -> i32 {
        let digits: Vec<i32> = n.to_string()
            .chars()
            .map(|c| c as i32 - '0' as i32)
            .collect();
        
        fn dp(digits: &Vec<i32>, pos: usize, tight: bool, different: bool) -> i32 {
            if pos == digits.len() {
                return if different { 1 } else { 0 };
            }
            
            let limit = if tight { digits[pos] } else { 9 };
            let mut count = 0;
            
            for d in 0..=limit {
                match d {
                    3 | 4 | 7 => continue,
                    _ => {
                        let new_tight = tight && d == limit;
                        let new_different = different || matches!(d, 2 | 5 | 6 | 9);
                        count += dp(digits, pos + 1, new_tight, new_different);
                    }
                }
            }
            count
        }
        
        dp(&digits, 0, true, false)
    }
}