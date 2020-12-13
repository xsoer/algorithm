/*
 * @lc app=leetcode id=2 lang=rust
 *
 * [2] Add Two Numbers
 */

// @lc code=start
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut carry = 0;
        let mut sum = 0;
        let (mut l1, mut l2) = (l1, l2);
        let mut res = None;
        let mut t = &mut res;
        // 从头遍历，匹配
        loop {
            match (l1, l2) {
                (Some(v1), Some(v2)) => {
                    sum = v1.val + v2.val;
                    l1 = v1.next;
                    l2 = v2.next;
                }
                (Some(v1), None) => {
                    sum = v1.val;
                    l1 = v1.next;
                    l2 = None;
                }
                (None, Some(v2)) => {
                    sum = v2.val;
                    l1 = None;
                    l2 = v2.next;
                }
                (None, None) => {
                    break;
                }
            }

            // if carry != 0 {
            //     sum += carry;
            //     carry = 0;
            // }
            // if sum > 9 {
            //     carry = 1;
            //     sum = sum - 10;
            // }
            sum += carry;
            carry = sum / 10;
            sum %= 10;

            // 赋值给
            *t = Some(Box::new(ListNode::new(sum)));
            if let Some(t_node) = t {
                t = &mut t_node.next
            }
        }
        if carry != 0 {
            *t = Some(Box::new(ListNode::new(carry)));
        }
        res
    }
}
// @lc code=end
