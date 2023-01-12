namespace Problems;

public class P2421_NumberOfGoodPaths {
    private class UnionFind {
        private int[] parent;
        private int[] rank;

        public UnionFind(int size) {
            parent = new int[size];
            rank = new int[size];
            for (int i = 0; i < size; i++) {
                parent[i] = i;
            }
        }
        public int Find(int x)
        {
            if (parent[x] != x) parent[x] = Find(parent[x]);
            return parent[x];
        }
        public void UnionSet(int x, int y)
        {
            int xset = Find(x), yset = Find(y);
            if (xset == yset) {
                return;
            }
            else if (rank[xset] < rank[yset]) {
                parent[xset] = yset;
            }
            else if (rank[xset] > rank[yset]) {
                parent[yset] = xset;
            }
            else {
                parent[yset] = xset;
                rank[xset]++;
            }
        }
    }

    public int NumberOfGoodPaths(int[] vals, int[][] edges) {
        int n = vals.Length;
        var adj = new List<int>[n];
        foreach (var edge in edges) {
            (adj[edge[0]] ??= new List<int>()).Add(edge[1]);
            (adj[edge[1]] ??= new List<int>()).Add(edge[0]);
        }
        // Mapping from value to all the nodes having the same value in non-descending order of values.
        var valuesToNodes = vals.Select((val, node) => (node, val)).Aggregate(new SortedDictionary<int, List<int>>(), (dict, node_val) => {
            var (node, val) = node_val;
            if (dict.TryGetValue(val, out var nodes)) {
                nodes.Add(node);
            }
            else {
                dict.Add(val, new List<int>() {node});
            }
            return dict;
        });

        var dsu = new UnionFind(n);
        int goodPaths = 0;

        // Iterate over all the nodes with the same value in sorted order, starting from the lowest
        // value.
        foreach (var (value, nodes) in valuesToNodes) {
            // For every node in nodes, combine the sets of the node and its neighbors into one set.
            foreach (var node in nodes) {
                foreach (var neighbor in adj[node] ?? Enumerable.Empty<int>()) {
                    // Only choose neighbors with a smaller value, as there is no point in
                    // traversing to other neighbors.
                    if (vals[node] >= vals[neighbor]) {
                        dsu.UnionSet(node, neighbor);
                    }
                }
            }
            // Map to compute the number of nodes under observation (with the same values) in each
            // of the sets.
            var group = new Dictionary<int, int>();
            // Iterate over all the nodes. Get the set of each node and increase the count of the
            // set by 1.
            foreach (var u in nodes) {
                var key = dsu.Find(u);
                if (group.ContainsKey(key)) {
                    group[key]++;
                }
                else {
                    group.Add(key, 1);
                }
            }
            // For each set of "size", add size * (size + 1) / 2 to the number of goodPaths.
            foreach (var size in group.Values) {
                goodPaths += (size * (size + 1) / 2);
            }
        }
        return goodPaths;
    }

    [Theory]
    [InlineData("[1]", "[]", 1)]
    [InlineData("[1,1,2,2,3]", "[[0,1],[1,2],[2,3],[2,4]]", 7)]
    [InlineData("[1,3,2,1,3]", "[[0,1],[0,2],[2,3],[2,4]]", 6)]
    public void Solve(string vals, string edges, int expected)
    {
        Assert.Equal(expected, NumberOfGoodPaths(vals.Trim('[', ']').Split(',').Select(el => int.Parse(el)).ToArray(), Util.ParsePairs(edges)));
    }     
}