namespace Problems;

public class P0005_LongestPalindromicSubstring
{
    public string LongestPalindrome(string s) {
        static int Expand(string s, int l, int r) {
            var n = s.Length;
            while (l>=0 && r<n && s[l]==s[r]) {
                l--;
                r++;
            }
            return r - l - 1;
        }
        var n = s.Length;
        var maxLen = 0;
        var center = 0;
        for (var i=0; i<n; i++) {
            var oddLen = Expand(s, i, i);
            var evenLen = Expand(s, i, i + 1);
            var currentMaxLen = Math.Max(oddLen, evenLen);
            if (maxLen < currentMaxLen) {
                maxLen = currentMaxLen;
                center = i;
            }
        }
        if (maxLen%2==0) {
            center++;
        }
        var start = center - (maxLen >> 1);
        return s.Substring(start, maxLen);
    }

    [Theory]
    [InlineData("abcabcbb", "bcb")]
    public void Solve(string s, string expected)
    {
        Assert.Equal(expected, LongestPalindrome(s));
    }
}