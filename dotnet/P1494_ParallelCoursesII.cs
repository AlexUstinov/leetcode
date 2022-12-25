namespace Problems;

public class P1494_ParallelCoursesII
{
    public int MinNumberOfSemesters(int n, int[][] relations, int k) {
        var g = relations.ToLookup(r => r[0] - 1, r => r[1] - 1);

        var dp = new int?[1 << n];
        dp[dp.Length - 1] = 0;
        return DpSolve(dp, 0, n, k, g);

        static int DpSolve(int?[] dp, int takenMask, int n, int k, ILookup<int, int> g) {
            if (dp[takenMask] is var cached && cached.HasValue) {
                return cached.Value;
            }
            var availableCourses = GetAvailableCourses(takenMask, n, g);
            if (availableCourses.Count <= k) {
                var nextMask = availableCourses.Aggregate(0, (mask, course) => mask | 1 << course);
                return (dp[takenMask] = 1 + DpSolve(dp, takenMask | nextMask, n, k, g)).Value;
            }
            var allocations = Enumerable.Range(0, 1 << availableCourses.Count).Where(a => BitCount(a)==k);
            foreach (var allocation in allocations) {
                var nextMask = 0;
                foreach (var idx in Enumerable.Range(0, availableCourses.Count)) {
                    if ((allocation & 1 << idx) != 0) {
                        nextMask |= 1 << availableCourses[idx];
                    }
                }
                if (1 + DpSolve(dp, takenMask | nextMask, n, k, g) is var solution && (dp[takenMask] is null || solution < dp[takenMask])) {
                    dp[takenMask] = solution;
                }
            }
            return dp[takenMask]!.Value;
        }

        static int BitCount(int n) {
            int count = 0;
            while (n!=0) {
                n = n & (n-1);
                count++;
            }
            return count;
        }

        static IReadOnlyList<int> GetAvailableCourses(int takenMask, int n, ILookup<int,int> g) {
            var indegree = new int[n];
            foreach (var v in Enumerable.Range(0, n).Where(v => (takenMask & 1 << v) == 0)) {
                foreach (var course in g[v].Where(course => (takenMask & 1 << course) == 0)) {
                    indegree[course] += 1;
                }
            }
            return Enumerable.Range(0, n).Where(course => (takenMask & 1 << course) == 0 && indegree[course]==0).ToList();
        }
    }

    [Theory]
    [InlineData(4, "[[2,1],[3,1],[1,4]]", 2, 3)]
    [InlineData(5, "[[2,1],[3,1],[4,1],[1,5]]", 2, 4)]
    public void Solve(int n, string relations, int k, int expected)
    {
        Assert.Equal(expected, MinNumberOfSemesters(n, Util.ParsePairs(relations), k));
    }         
}