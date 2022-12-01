namespace Problems.DataStructures;

using System.Linq;
public class TreeNode
{
    public int val;
    public TreeNode? left;
    public TreeNode? right;
    public TreeNode(int val = 0, TreeNode? left = null, TreeNode? right = null)
    {
        this.val = val;
        this.left = left;
        this.right = right;
    }

    public static TreeNode? FromString(string values)
    {
        return FromNumbers(values
            .Trim(' ', '[', ']')
            .Split(',')
            .Select(token => Int32.TryParse(token.Trim(), out var num) ? num : default(int?)));
    }

    public static TreeNode? FromNumbers(IEnumerable<int?> numbers)
    {
        var enumerator = numbers?.GetEnumerator();
        if (enumerator == null || !enumerator.MoveNext()) {
            return null;
        }
        var root = new TreeNode(enumerator.Current ?? throw new InvalidOperationException("Root must have a value."));
        var queue = new Queue<TreeNode>();
        queue.Enqueue(root);
        bool isLeft = true;
        while (enumerator.MoveNext()) {
            var childNode = enumerator.Current.HasValue ? new TreeNode(enumerator.Current.Value) : null;
            if (isLeft) {
                var node = queue.Peek();
                node.left = childNode;
            }
            else {
                var node = queue.Dequeue();
                node.right = childNode;
            }
            if (childNode != null) {
                queue.Enqueue(childNode);
            }
            isLeft ^= true;
        }
        return root;
    }

    public override string ToString()
    {
        return "[" + string.Join(',', ToNumbers()) + "]";
    }

    public IEnumerable<int?> ToNumbers()
    {
        var queue = new Queue<TreeNode?>();
        queue.Enqueue(this);
        var valCount = 1;
        while (valCount > 0) {
            var node = queue.Dequeue();
            if (node == null) {
                yield return null;
            }
            else {
                yield return node.val;
                if (node.left != null) {
                    valCount++;
                }
                if (node.right != null) {
                    valCount++;
                }
                queue.Enqueue(node.left);
                queue.Enqueue(node.right);
            }
            valCount--;
        }
    }

}