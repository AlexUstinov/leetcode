using System.Text;

namespace Problems;

public class P0006_ZigZagConversion
{
    public string Convert(string s, int numRows)
    {
        var result = new StringBuilder(s.Length);
        foreach (var c in EnumerateZigZag(s, numRows)) {
            result.Append(c);
        }
        return result.ToString();
    }

    private static IEnumerable<char> EnumerateZigZag(string s, int numRows)
    {
        var len = s.Length;
        var zigZagLen = numRows > 1 ? 2 * numRows - 2 : 1;
        for (var row = 0; row < numRows; row++) {
            var isInnerRow = row > 0 && row < numRows - 1;
            for (var idx = 0; idx + row is var idx1 && idx1 < len; idx += zigZagLen) {
                yield return s[idx1];
                if (isInnerRow && idx + zigZagLen - row is var idx2 && idx2 < len) {
                    yield return s[idx2];
                }
            }
        }
    }

    [Theory]
    [InlineData("PAYPALISHIRING", 1, "PAYPALISHIRING")]
    [InlineData("PAYPALISHIRING", 2, "PYAIHRNAPLSIIG")]
    [InlineData("PAYPALISHIRING", 3, "PAHNAPLSIIGYIR")]
    [InlineData("PAYPALISHIRING", 4, "PINALSIGYAHRPI")]
    [InlineData("PAYPALISHIRING", 13, "PAYPALISHIRIGN")]
    [InlineData("PAYPALISHIRING", 14, "PAYPALISHIRING")]
    [InlineData("PAYPALISHIRING", 1000, "PAYPALISHIRING")]
    public void Solve(String s, int numRows, string expected)
    {
        Assert.Equal(expected, Convert(s, numRows));
    }
}