/*
 * @lc app=leetcode.cn id=4 lang=rust
 *
 * [4] 寻找两个正序数组的中位数
 */

/* 
    数组合并

*/

// @lc code=start
impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut all: Vec<i32> = vec![];
        all.extend(nums1);
        all.extend(nums2);
        all.sort();
        let len = all.len();
        let mut res = 0.0 as f64;
        let div = len % 2;
        println!("div {:#?}", all);
        let min = len / 2 as usize;
        
        if div == 0 {
            res = ((all[min-1] + all[min]) / 2) as f64;
        } else {
            res = all[min] as f64;
        }
        res
    }
}
// @lc code=end

