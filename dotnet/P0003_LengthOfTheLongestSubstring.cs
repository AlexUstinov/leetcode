namespace Problems;

public class P0003_LengthOfTheLongestSubstring
{
    public class Solution
    {
        public int LengthOfLongestSubstring(string s)
        {
            int maxLength = 0, length = 0, startIdx = 0;
            var charIndexes = new Dictionary<char, int>();
            for (int index = 0, sLength = s.Length; index < sLength; index++) {
                var c = s[index];
                if (!charIndexes.TryGetValue(c, out var lastIdx) || lastIdx < startIdx) {
                    length++;
                }
                else {
                    startIdx = lastIdx + 1;
                    if (length > maxLength) {
                        maxLength = length;
                    }
                    length = index - lastIdx;
                }
                charIndexes[c] = index;
            }
            return maxLength < length ? length : maxLength;
        }
    }

    [Theory]
    [InlineData("abcabcbb", 3)]
    [InlineData("bbbbb", 1)]
    [InlineData("pwwkew", 3)]
    public void Solve(string s, int answer)
    {
        Assert.Equal(answer, new Solution().LengthOfLongestSubstring(s));
    }
}