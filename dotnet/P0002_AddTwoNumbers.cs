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
        public ListNode AddTwoNumbers(ListNode l1, ListNode l2)
        {
            var carryOver = 0;
            ListNode? num1 = l1, num2 = l2, head = null, tail = null;
            while (num1!=null || num2!=null) {
                var sum = (num1?.val ?? 0) + (num2?.val ?? 0) + carryOver;
                carryOver = sum > 9 ? 1 : 0;
                var node = new ListNode(sum % 10);
                if (tail==null) {
                    head = tail = node;
                }
                else {
                    tail = tail.next = node;
                }
                num1 = num1?.next;
                num2 = num2?.next;
            }
            if (tail!=null && carryOver > 0) {
                tail.next = new ListNode(carryOver);
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