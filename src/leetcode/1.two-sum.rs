/*
 * @lc app=leetcode id=1 lang=rust
 *
 * [1] Two Sum
 */

// @lc code=start
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let len = nums.len();
        for i in 0..len {
            for j in i+1..len {
                if nums[i] + nums[j] == target {
                    return vec![j as i32, i as i32];
                }
            }
        }
        vec![]
    }
}
// @lc code=end

