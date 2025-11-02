/*
 * @lc app=leetcode id=9 lang=rust
 *
 * [9] Palindrome Number
 */

// @lc code=start
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }
        let mut num = x;
        let mut rev = 0;
        while num > 0 {
            rev = rev * 10 + num % 10;
            num /= 10;
        }
        x == rev
    }
}
// @lc code=end

