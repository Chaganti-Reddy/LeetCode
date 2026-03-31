impl Solution {
    pub fn generate_string(str1: String, str2: String) -> String {
        let s1: Vec<char> = str1.chars().collect();
        let s2: Vec<char> = str2.chars().collect();
        let n = s1.len();
        let m = s2.len();
        let mut word: Vec<char> = vec![' '; n + m - 1];

        for i in 0..n {
            if s1[i] == 'T' {
                for j in 0..m {
                    if word[i + j] == ' ' {
                        word[i + j] = s2[j];
                    } else if word[i + j] != s2[j] {
                        return "".to_string();
                    }
                }
            }
        }

        for i in 0..n {
            if s1[i] == 'F' {
                let matches = (0..m).all(|j| {
                    let c = if word[i + j] == ' ' { 'a' } else { word[i + j] };
                    c == s2[j]
                });

                if matches {
                    let fix = (0..m).rev().find(|&j| word[i + j] == ' ');
                    match fix {
                        Some(j) => word[i + j] = (s2[j] as u8 + 1) as char,
                        None => return "".to_string(),
                    }
                }
            }
        }

        word.iter_mut().for_each(|c| if *c == ' ' { *c = 'a' });

        word.iter().collect()
    }
}