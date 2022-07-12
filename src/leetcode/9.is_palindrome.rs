/*
 * @lc app=leetcode.cn id=9 lang=rust
 *
 * [9] 回文数
 */

// @lc code=start
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let a = x.to_string();
        let b = a.chars().rev().collect::<String>();
        a == b
    }
}
// @lc code=end

