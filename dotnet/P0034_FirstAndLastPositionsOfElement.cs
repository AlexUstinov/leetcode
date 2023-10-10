using Problems.DataStructures;

namespace Problems;

public class P0034_FirstAndLastPositionsOfElement {
    private static int Partition(int[] nums, Func<int, bool> cmp) {
        var (l, r) = (0, nums.Length);
        while (l < r) {
            var m = l + ((r - l) / 2);
            if (cmp(nums[m])) {
                l = m + 1;
            }
            else {
                r = m;
            }
        }
        return l;
    }
    public int[] SearchRange(int[] nums, int target) {
        var l_bound = Partition(nums, (el) => el < target);
        var r_bound = Partition(nums, (el) => el <= target);
        return l_bound==r_bound
            ? new[] { -1, -1 }
            : new[] { l_bound, r_bound - 1 };
    }

    [Theory]
    [InlineData("[5,7,7,8,8,10]", 8, "[3,4]")]
    public void Solve(string nums, int target, string expected)
    {
        var numList = ListNode.FromString(nums);
        var expectedList = ListNode.FromString(expected);

        Assert.Equal(expectedList?.ToNumbers(), SearchRange(numList?.ToNumbers()?.ToArray()!, target));
    }
}