using Problems.DataStructures;

namespace Problems;

public class P0328_OddEvenLinkedList
{
    public ListNode? OddEvenList(ListNode? head) {
        var pointer = head;
        var odd = new ListNode();
        var even = new ListNode();
        var (first, second) = (odd, even);
        var counter = 0;
        while (pointer!=null) {
            first.next = pointer;
            (first, second) = (second, pointer);
            pointer = pointer.next;
            counter++;
        }
        first.next = second.next = null;
        var oddTail = counter % 2 == 0 ? first : second;
        oddTail.next = even.next;
        return odd.next;
    }

    [Theory]
    [InlineData("[]", "[]")]
    [InlineData("[1,2,3,4,5]", "[1,3,5,2,4]")]
    [InlineData("[2,1,3,5,6,4,7]", "[2,3,6,7,1,5,4]")]
    public void Solve(string list, string expected)
    {
        var listNode = ListNode.FromString(list);
        var expectedList = ListNode.FromString(expected);
        Assert.Equal(expectedList?.ToNumbers(), OddEvenList(listNode)?.ToNumbers());
    }
}