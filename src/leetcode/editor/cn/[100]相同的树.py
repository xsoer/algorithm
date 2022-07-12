# 给你两棵二叉树的根节点 p 和 q ，编写一个函数来检验这两棵树是否相同。 
# 
#  如果两个树在结构上相同，并且节点具有相同的值，则认为它们是相同的。 
# 
#  
# 
#  示例 1： 
# 
#  
# 输入：p = [1,2,3], q = [1,2,3]
# 输出：true
#  
# 
#  示例 2： 
# 
#  
# 输入：p = [1,2], q = [1,null,2]
# 输出：false
#  
# 
#  示例 3： 
# 
#  
# 输入：p = [1,2,1], q = [1,1,2]
# 输出：false
#  
# 
#  
# 
#  提示： 
# 
#  
#  两棵树上的节点数目都在范围 [0, 100] 内 
#  -10⁴ <= Node.val <= 10⁴ 
#  
#  Related Topics 树 深度优先搜索 广度优先搜索 二叉树 👍 858 👎 0


# leetcode submit region begin(Prohibit modification and deletion)
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def isSameTree(self, p: TreeNode, q: TreeNode) -> bool:
        # 先生成tree list,然后在每个元素做判断
        # return self.travel_list(p, q)

        # 遍历时比较
        return self.travel_cmp(p, q)

    def travel_cmp(self, p: TreeNode, q: TreeNode) -> bool:
        if not p and not q:
            return True
        if not p and q:
            return False
        if p and not q:
            return False
        if p.val != q.val:
            return False
        return self.travel_cmp(p.left, q.left) and self.travel_cmp(p.right, q.right)

    def travel_list(self, p: TreeNode, q: TreeNode) -> bool:
        if not p and not q:
            return True
        if not p and q:
            return False
        if p and not q:
            return False
        p_l = self._travel_tree(p)
        q_l = self._travel_tree(q)
        if len(p_l) != len(q_l):
            return False
        for i in range(0, len(p_l)):
            if p_l[i] != q_l[i]:
                return False
        return True

    def _travel_tree(self, tree: TreeNode) -> List[int]:
        res = [tree.val]
        if tree.left:
            res.extend(self._travel_tree(tree.left))
        else:
            res.append(None)
        if tree.right:
            res.extend(self._travel_tree(tree.right))
        else:
            res.append(None)
        return res
# leetcode submit region end(Prohibit modification and deletion)
