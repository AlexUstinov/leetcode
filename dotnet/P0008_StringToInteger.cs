namespace Problems;

public class P0008_StringToInteger
{
    public int MyAtoi(string s) {
        const char zeroChar = '0';
        s = s.TrimStart();
        if (string.IsNullOrEmpty(s)) {
            return 0;
        }
        var isNegative = s[0] == '-';
        var isPositiveSign = s[0] == '+';
        long result = 0;
        foreach (var digitChar in s.Skip(isNegative || isPositiveSign ? 1 : 0)) {
            int digit = digitChar - zeroChar;
            if (0<=digit && digit<=9) {
                result *= 10;
                if (isNegative) {
                    result -= digit;
                    if (result <= int.MinValue) {
                        return int.MinValue;
                    }
                }
                else {
                    result += digit;
                    if (result >= int.MaxValue) {
                        return int.MaxValue;
                    }
                }
            }
            else {
                break;
            }
        }
        return (int) result;
    }

    [Theory]
    [InlineData("", 0)]
    [InlineData(" ", 0)]
    [InlineData("abc", 0)]
    [InlineData("123", 123)]
    [InlineData("-124", -124)]
    [InlineData("  +125abc", 125)]
    [InlineData("3000000000", Int32.MaxValue)]
    [InlineData("-3000000000", Int32.MinValue)]
    public void Solve(string s, int expected)
    {
        Assert.Equal(expected, MyAtoi(s));
    }
}