use std::collections::HashSet;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut cSet = HashSet::new();
        let mut l: usize = 0;
        let mut res: i32 = 0;
        for (i, c) in s.chars().enumerate(){
            while cSet.contains(&c){
                cSet.remove(&s.chars().nth(l).unwrap());
                l += 1;
            }
            cSet.insert(c);
            res = res.max(i as i32 - l as i32 + 1);
        }
        res
    }
}