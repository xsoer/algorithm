/*
 * @lc app=leetcode.cn id=3 lang=rust
 *
 * [3] 无重复字符的最长子串
 */

// String和slice的用法
// String不能index
// 字符串的截取
// 判断字符是否在字符串内
// 遍历

// @lc code=start
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        use std::cmp::max;
        /* let mut max_len = 0 as i32;
        for i in 0..s.len() {
            let mut tmp_len = 0 as i32;
            let mut sub = String::new();
            for j in i..s.len() {
                let a = s.chars().nth(j).unwrap();
                if sub.contains(a) {
                    break;
                }
                sub.push(a);
                tmp_len += 1;
                if tmp_len > max_len {
                    max_len = tmp_len;
                }
            }
        } */
        
        if s == "" {
            return 0;
        }

        // 窗口法
        let mut res = 1;
        let mut start = 0;
        let mut end = 1;
        let len = s.len();

        while end < len {
            if !&s[start..end].contains(&s[end..end+1]) {
                end += 1;
                res = max(res, s[start..end].len());
            } else {
                start += 1;
                if start >= end {
                    end = start + 1;
                }
            }
        }
        res as i32
    }
}
// @lc code=end
