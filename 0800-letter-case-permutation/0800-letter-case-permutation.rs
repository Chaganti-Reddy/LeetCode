impl Solution {
    fn backtrack(chars: &mut Vec<char>, i: usize, ans: &mut Vec<String>) {
        if i == chars.len() {
            ans.push(chars.iter().collect());
            return;
        }

        if chars[i].is_ascii_digit() {
            Self::backtrack(chars, i + 1, ans);
        } else {
            chars[i] = chars[i].to_ascii_lowercase();
            Self::backtrack(chars, i + 1, ans);

            chars[i] = chars[i].to_ascii_uppercase();
            Self::backtrack(chars, i + 1, ans);
        }
    }

    pub fn letter_case_permutation(s: String) -> Vec<String> {
        let mut ans = Vec::new();
        let mut chars: Vec<char> = s.chars().collect();
        Self::backtrack(&mut chars, 0, &mut ans);
        ans
    }
}
