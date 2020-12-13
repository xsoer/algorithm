/*
 * @lc app=leetcode id=1 lang=rust
 *
 * [1] Two Sum
 * 用HashMap或者BTreeMap
 */
// @lc code=start
use std::collections::BTreeMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: BTreeMap<i32, usize> = BTreeMap::new();
        for (i, v) in nums.iter().enumerate() {
            match map.get(&(target - v)) {
                None => {
                    map.insert(*v, i);
                }
                Some(pos) => {
                    return vec![*pos as i32, i as i32];
                }
            }
        }
        vec![]
    }
}
// @lc code=end
