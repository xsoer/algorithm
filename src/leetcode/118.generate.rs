/*
 * @lc app=leetcode.cn id=118 lang=rust
 *
 * [118] 杨辉三角
 */

// @lc code=start
impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let rows = num_rows as usize;
        let mut res: Vec<Vec<i32>> = vec![];
        for i in 0..rows {
            let mut row = vec![];
            for j in 0..i+1 {
                if j == 0 || j == i {
                    row.push(1);
                } else {
                    row.push(res[i-1][j-1] + res[i-1][j]);
                }
            }
            res.push(row);
        }
        res
    }
}
// @lc code=end

