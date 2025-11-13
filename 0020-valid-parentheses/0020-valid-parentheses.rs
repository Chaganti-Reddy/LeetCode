use std::collections::HashMap;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let map: HashMap<char, char> = 
            [('(', ')'), ('[', ']'), ('{', '}')]
            .into_iter()
            .collect();

        let mut stack: Vec<char> = Vec::new();

        for ch in s.chars() {
            if map.contains_key(&ch) {
                stack.push(ch);
            } else {
                match stack.pop() {
                    Some(open) => {
                        if map.get(&open) != Some(&ch) {
                            return false;
                        }
                    }
                    None => return false,
                }
            }
        }
        stack.is_empty()
    }
}
