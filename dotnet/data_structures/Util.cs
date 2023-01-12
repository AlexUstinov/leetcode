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

    public static int[] ParseValues(string vals) => vals.Trim('[', ']').Split(',').Select(el => int.Parse(el)).ToArray();
}