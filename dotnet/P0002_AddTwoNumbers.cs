namespace Problems;

public class P0002_AddTwoNumbers
{
    public class ListNode
    {
        public int val;
        public ListNode next;
        public ListNode(int val = 0, ListNode next = null)
        {
            this.val = val;
            this.next = next;
        }
    }

    private class Solution
    {
        public ListNode AddTwoNumbers(ListNode l1, ListNode l2)
        {
            return null;
        }
    }
    private static IEnumerable<object[]> GetTestCases()
    {
        return Enumerable.Empty<object[]>();
    }

    [Theory]
    [MemberData(nameof(GetTestCases))]
    public void Solve(ListNode l1, ListNode l2, ListNode expected)
    {

    }
}