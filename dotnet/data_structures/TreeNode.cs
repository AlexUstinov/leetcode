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
        const int MaxCount = 15;
        var numbers = new List<string>(MaxCount + 1);
        var enumerator = ToNumbers().GetEnumerator();
        while (numbers.Count < MaxCount && enumerator.MoveNext()) {
            numbers.Add(enumerator.Current.ToString() ?? "null");
        }
        if (enumerator.MoveNext()) {
            numbers.Add("...");
        }
        return "[" + string.Join(',', numbers) + "]";
    }

    public string ToFullString() => "[" + string.Join(',', ToNumbers().Select(n => n?.ToString() ?? "null")) + "]";

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
                valCount--;
                if (node.left != null) {
                    valCount++;
                }
                if (node.right != null) {
                    valCount++;
                }
                queue.Enqueue(node.left);
                queue.Enqueue(node.right);
            }
        }
    }
}

public class TreeNodeTests
{
    [Fact]
    public void ToNumbersMethodWorksCorrectly_1()
    {
        var numbers = new[] { 10, 5, 15, 3, 7, default(int?), 18 };
        var root = TreeNode.FromNumbers(numbers);
        Assert.Equal(numbers, root!.ToNumbers());
    }

    [Fact]
    public void ToNumbersMethodWorksCorrectly_2()
    {
        var numbers = new int?[] { 10, 5, 15, 3, 7, 18 };
        var root = TreeNode.FromNumbers(numbers);
        Assert.Equal(numbers, root!.ToNumbers());
    }

    [Fact]
    public void ToNumbersMethodWorksCorrectly_3()
    {
        var numbers = new int?[] { 10, 5, 15, 3, 7, 18, 1, null, 6 };
        var root = TreeNode.FromNumbers(numbers);
        Assert.Equal(numbers, root!.ToNumbers());
    }

    [Fact]
    public void ToStringMethodDoesntTruncateTreeWithLessThan16Nodes()
    {
        const string tree = "[1,2,3,4,5,6,7,8,9,10,11,12,13,14,15]";
        var root = TreeNode.FromString(tree);
        Assert.Equal(tree, root!.ToString());
    }

    [Fact]
    public void ToStringMethodTruncatesTreesWithOver15Nodes()
    {
        const string tree = "[1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16]";
        const string truncatedTree = "[1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,...]";
        var root = TreeNode.FromString(tree);
        Assert.Equal(truncatedTree, root!.ToString());
    }

    [Theory]
    [InlineData("[10,5,15,3,7,null,18]")]
    [InlineData("[10,5,15,3,7,13,18,1,null,6]")]
    [InlineData("[1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16]")]
    public void ToFullStringMethodSerializesTreeProperly(string tree)
    {
        var root = TreeNode.FromString(tree);
        Assert.Equal(tree, root!.ToFullString());
    }
}