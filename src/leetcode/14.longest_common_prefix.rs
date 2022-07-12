/*
 * @lc app=leetcode.cn id=14 lang=rust
 *
 * [14] 最长公共前缀
 */

// @lc code=start
impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.len() == 0 {
            return String::new();
        }
        let mut min_len = strs[0].len();
        for i in 1..strs.len() {
            if strs[i].len() < min_len {
                min_len = strs[i].len();
            }
        }
        let mut res = String::new();
        for i in 0..min_len {
            let mut flag = true;
            for j in 0..strs.len() {
                if strs[j].as_bytes()[i] != strs[0].as_bytes()[i] {
                    flag = false;
                    break;
                }
            }
            if flag {
                res.push(strs[0].as_bytes()[i] as char);
            } else {
                break;
            }
        }
        res
    }
}
// @lc code=end

