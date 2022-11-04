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
        public ListNode? AddTwoNumbers(ListNode? num1, ListNode? num2)
        {
            static ListNode? AddNumbers(ListNode? num1, ListNode? num2, int carry) => (num1, num2) switch {
                (num1: not null, num2: not null) when num1.val + num2.val + carry is var sum => new ListNode(sum % 10, AddNumbers(num1.next, num2.next, sum / 10)),
                _ when (num1 ?? num2) is var num && num is not null && num.val + carry is var sum => new ListNode(sum % 10, AddNumbers(num.next, null, sum / 10)),
                _ => carry > 0 ? new ListNode(carry) : null
            };

            return AddNumbers(num1, num2, 0);
        }
    }

    private static IEnumerable<object[]> GetTestCases()
    {
        return new[] {
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