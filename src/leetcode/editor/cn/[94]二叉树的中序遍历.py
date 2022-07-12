#  给定一个二叉树的根节点 root ，返回 它的 中序 遍历 。
#
#
#
#  示例 1：
#
#
#  输入：root = [1,null,2,3]
#  输出：[1,3,2]
#
#
#  示例 2：
#
#
#  输入：root = []
#  输出：[]
#
#
#  示例 3：
#
#
#  输入：root = [1]
#  输出：[1]
#
#
#
#
#  提示：
#
#
#  树中节点数目在范围 [0, 100] 内
#  -100 <= Node.val <= 100
#
#
#
#
#  进阶: 递归算法很简单，你可以通过迭代算法完成吗？
#  Related Topics 栈 树 深度优先搜索 二叉树 👍 1470 👎 0
#


# leetcode submit region begin(Prohibit modification and deletion)
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def inorderTraversal(self, root: Optional[TreeNode]) -> List[int]:
        # 递归法
        # return self.recursion(root)

        # 颜色标注法
        return self.color(root)

    def recursion(self, root: Optional[TreeNode]) -> List[index]:
        # 判断是否为空
        if not root:
            return []
        result = []
        # 判断是否有左节点
        if root.left:
            result.extend(self.inorderTraversal(root.left))
        result.append(root.val)
        # 判断是否有右节点
        if root.right:
            result.extend(self.inorderTraversal(root.right))
        return result

    def color(self, root: Optional[TreeNode]) -> List[index]:
        if not root:
            return []
        WHITE, GRAY = 0, 1
        res = []
        stack = [(WHITE, root)]
        while stack:
            color, node = stack.pop()
            if node is None:
                continue
            if color == WHITE:
                stack.append((WHITE, node.right))
                stack.append((GRAY, node))
                stack.append((WHITE, node.left))
            else:
                res.append(node.val)
        return res

# leetcode submit region end(Prohibit modification and deletion)
