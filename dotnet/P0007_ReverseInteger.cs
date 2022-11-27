namespace Problems;

public class P0007_ReverseInteger
{
    public int Reverse(int number)
    {
        const int upperLim = Int32.MaxValue / 10;
        const int lowerLim = Int32.MinValue / 10;
        var isNegative = number < 0;
        var result = 0;
        while(number != 0) {
            var q = number / 10;
            var r = number - q * 10;
            if (isNegative) {
                if (result < lowerLim || result == lowerLim && r < -8) {
                    return 0;
                }
            }
            else {
                if (result > upperLim || result == upperLim && r > 7) {
                    return 0;
                }
            }
            result = result * 10 + r;
            number = q;
        }
        return result;
    }

    [Theory]
    [InlineData(123, 321)]
    [InlineData(-321, -123)]
    [InlineData(Int32.MaxValue, 0)]
    [InlineData(Int32.MinValue, 0)]
    public void Solve(int number, int expected)
    {
        Assert.Equal(expected, Reverse(number));
    }
}