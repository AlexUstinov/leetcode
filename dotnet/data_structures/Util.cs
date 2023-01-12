namespace Problems;

public static class Util
{
    public static int[][] ParsePairs(string pairs)
    {
        return pairs
            .Trim(' ', '[', ']')
            .Split("],[")
            .Where(token => !string.IsNullOrWhiteSpace(token))
            .Select(row => row
                .Trim(' ', '[', ']')
                .Split(',', 2)
                .Select(token => int.Parse(token))
                .ToArray())
            .ToArray();
    }
}