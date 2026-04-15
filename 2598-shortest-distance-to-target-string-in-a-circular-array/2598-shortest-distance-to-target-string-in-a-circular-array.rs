impl Solution {
    pub fn closest_target(words: Vec<String>, target: String, start_index: i32) -> i32 {
        let n = words.len() as i32;
        let mut ans = i32::MAX;

        for (i, word) in words.iter().enumerate() {
            if word == &target {
                let i = i as i32;
                let direct = (start_index - i).abs();
                let circular = n - direct;
                ans = ans.min(direct.min(circular));
            }
        }

        if ans == i32::MAX { -1 } else { ans }
    }
}