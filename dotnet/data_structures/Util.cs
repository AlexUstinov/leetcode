namespace Problems;

public static class Util
{
    public static int[][] ParsePairs(string pairs)
    {
        return pairs
            .Trim(' ', '[', ']')
            .Split("],[")
            .Select(row => row
                .Trim(' ', '[', ']')
                .Split(',', 2)
                .Select(token => int.Parse(token))
                .ToArray())
            .ToArray();
    }
}