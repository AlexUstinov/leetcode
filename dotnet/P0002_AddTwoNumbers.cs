namespace Problems;

public class P0002_AddTwoNumbers
{
    public class ListNode
    {
        public int val;
        public ListNode? next;
        public ListNode(int val = 0, ListNode? next = null)
        {
            this.val = val;
            this.next = next;
        }
    }

    private class Solution
    {
        public ListNode AddTwoNumbers(ListNode? num1, ListNode? num2)
        {
            ListNode head = new ListNode(), tail = head;
            ListNode? prevTail = null;
            while (num1!=null || num2!=null) {
                var sum = (num1?.val ?? 0) + (num2?.val ?? 0) + tail.val;
                tail.val = sum % 10;
                prevTail = tail;
                tail = tail.next = new ListNode(sum > 9 ? 1 : 0);
                num1 = num1?.next;
                num2 = num2?.next;
            }
            if (prevTail is not null && tail.val==0) {
                prevTail.next = null;
            }
            return head ?? new ListNode();
        }
    }

    private static IEnumerable<object[]> GetTestCases()
    {
        return new [] {
            new object[] {1, 2, 3},
            new object[] {1, 20 , 21},
            new object[] {1, 999, 1000}
        };
    }

    [Theory]
    [MemberData(nameof(GetTestCases))]
    public void Solve(int l1, int l2, int expected)
    {
        static ListNode ConvertToLinkedList(int num)
        {
            ListNode? head = null, tail = null;
            while (num > 0) {
                var node = new ListNode(num % 10);
                if (tail is null) {
                    head = tail = node;
                }
                else {
                    tail = tail.next = node;
                }
                num /= 10;
            }
            return head ?? new ListNode();
        }

        static int ConvertFromLinkedList(ListNode? head)
        {
            var num = 0;
            var multiple = 1;
            while (head is not null) {
                num += head.val * multiple;
                multiple *= 10;
                head = head.next;
            }
            return num;
        }

        var solution = new Solution();
        var num1 = ConvertToLinkedList(l1);
        var num2 = ConvertToLinkedList(l2);
        Assert.Equal(expected, ConvertFromLinkedList(solution.AddTwoNumbers(num1, num2)));
    }
}