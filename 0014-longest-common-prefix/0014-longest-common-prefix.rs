/*
 * @lc app=leetcode id=14 lang=rust
 *
 * [14] Longest Common Prefix
 */

// @lc code=start
impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut result = String::new();
        for i in 0..strs[0].len() {
            let mut flag = true;
            for j in 1..strs.len() {
                if Some(strs[j].chars().nth(i)) != Some(strs[0].chars().nth(i)) {
                    flag = false;
                    break;
                }
            }
            if flag {
                result.push(strs[0].chars().nth(i).unwrap());
            } else {
                break;
            }
        }
        result
    }
}
// @lc code=end

