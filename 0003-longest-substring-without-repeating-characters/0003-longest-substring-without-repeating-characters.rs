use std::collections::HashMap;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut last = HashMap::new();
        let mut l = 0;
        let mut res = 0;

        for (r, c) in s.chars().enumerate() {
            if let Some(&prev) = last.get(&c) {
                l = l.max(prev + 1);
            }
            last.insert(c, r);
            res = res.max((r - l + 1) as i32);
        }
        res
    }
}
