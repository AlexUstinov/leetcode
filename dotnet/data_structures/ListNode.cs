namespace Problems.DataStructures;
public class ListNode
{
    public int val;
    public ListNode? next;
    public ListNode(int val = 0, ListNode? next = null)
    {
        this.val = val;
        this.next = next;
    }

    public static ListNode? FromString(string values)
    {
        return FromNumbers(values
            .Trim(' ', '[', ']')
            .Split(',')
            .Select(token => Int32.Parse(token.Trim())));
    }

    public static ListNode? FromNumbers(IEnumerable<int> numbers)
    {
        return numbers
            .Aggregate<int, (ListNode? head, ListNode? tail)>((null, null), (list, val) => {
                list.tail = list.tail switch {
                    null => list.head = new ListNode(val),
                    _ => list.tail.next = new ListNode(val)
                };
                return list;
            }).head;
    }

    public override string ToString()
    {
        return "[" + string.Join(',', ToNumbers()) + "]";
    }

    public IEnumerable<int> ToNumbers() {
        var head = this;
        while(head != null) {
            yield return head.val;
            head = head.next;
        }
    }
}