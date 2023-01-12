using System.Diagnostics.CodeAnalysis;

namespace Problems;

public class P0057_InsertInterval {
    private class IntervalComparer: IComparer<int[]> {
        public static IntervalComparer Default {get;} = new IntervalComparer();

        private IntervalComparer() {}
        public int Compare(int[]? a, int[]? b) {
            ArgumentNullException.ThrowIfNull(a);
            ArgumentNullException.ThrowIfNull(b);
            return a[0].CompareTo(b[0]);
        }
    }

    public int[][] Insert(int[][] intervals, int[] newInterval) {
        var first = Array.BinarySearch(intervals, newInterval, IntervalComparer.Default);
        var last = Array.BinarySearch(intervals, new int[] { newInterval[1] }, IntervalComparer.Default);
        if (first<0) {
            first = ~first - 1;
            if (first >= 0 && intervals[first][1] >= newInterval[0]) {
                newInterval[0] = intervals[first][0];
            }
            else {
                first++;
            }
        }
        if (last<0) {
            last = ~last - 1;
            if (last >= 0 && newInterval[1] < intervals[last][1]) {
                newInterval[1] = intervals[last][1];
            }
        }
        else {
            newInterval[1] = intervals[last][1];
        }

        var result = new int[intervals.Length - (last - first)][];
        Array.Copy(intervals, 0, result, 0, first);
        result[first] = newInterval;
        Array.Copy(intervals, last + 1, result, first + 1, intervals.Length - (last + 1));
        return result;
    }

    private class IntervalEqComparer : IEqualityComparer<int[]>
    {
        public static IntervalEqComparer Default {get;} = new IntervalEqComparer();
        private IntervalEqComparer() {}
        public bool Equals(int[]? x, int[]? y) => x?.Length==2 && y?.Length==2 && (x[0], x[1]).Equals((y[0], y[1]));
        public int GetHashCode([DisallowNull] int[] x) => (x[0], x[1]).GetHashCode();
    }

    [Theory]
    [InlineData("[[1,3],[6,9]]", "[2,5]", "[[1,5],[6,9]]")]
    [InlineData("[[1,2],[3,5],[6,7],[8,10],[12,16]]", "[4,8]", "[[1,2],[3,10],[12,16]]")]
    [InlineData("[]", "[5,7]", "[[5,7]]")]
    public void Solve(string intervals, string newInterval, string expected) {
        Assert.Equal(Util.ParsePairs(expected), Insert(Util.ParsePairs(intervals), Util.ParseValues(newInterval)), IntervalEqComparer.Default);
    }
}