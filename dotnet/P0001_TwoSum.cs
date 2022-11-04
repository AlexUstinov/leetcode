namespace Problems;

public class P0001_TwoSum
{
    public class Solution
    {
        public int[] TwoSum(int[] nums, int target)
        {
            var length = nums.Length;
            var numIndexes = new Dictionary<int, int>(length);
            for (var index = 0; index < length; index++) {
                var num = nums[index];
                var complement = target - num;
                if (numIndexes.TryGetValue(complement, out var complementIndex)) {
                    return new[] { complementIndex, index };
                }
                numIndexes.Add(num, index);
            }
            throw new InvalidDataException("There is no solution for the input data provided.");
        }
    }

    private static IEnumerable<object[]> GetTestCases()
    {
        return new[] {
            new [] { new [] {2, 7, 11, 15}, (object)9,  new [] {0, 1} },
            new [] { new [] {3, 2, 4}, (object)6,  new [] {1, 2} },
            new [] { new [] {3, 3}, (object)6,  new [] {0, 1} }
        };
    }

    [Theory]
    [MemberData(nameof(GetTestCases))]
    public void Solve(int[] nums, int target, int[] answer)
    {
        Assert.Equal(answer, new Solution().TwoSum(nums, target).OrderBy(item => item));
    }
}