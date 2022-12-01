namespace Problems;

using Problems.DataStructures;

public class P0938_RangeSumOfBst
{
    public int RangeSumBST(TreeNode? root, int low, int high) {
        return root switch {
            TreeNode {val: var val, left: var l, right: var r} when val >= low && val <= high =>
                val + RangeSumBST(l, low, high) + RangeSumBST(r, low, high),
            TreeNode {val: var val, right: var r} when val < low => RangeSumBST(r, low, high),
            TreeNode {val: var val, left: var l} when val > high => RangeSumBST(l, low, high),
            _ => 0
        };
    }

    [Theory]
    [InlineData("[10,5,15,3,7,null,18]", 7, 15, 32)]
    [InlineData("[10,5,15,3,7,13,18,1,null,6]", 6, 10, 23)]
    public void Solve(string tree, int low, int high, int expected) {
        var root = TreeNode.FromString(tree);
        Assert.Equal(expected, RangeSumBST(root, low, high));
    }    
}