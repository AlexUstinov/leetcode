namespace Problems;

public class P0004_MedianOfTwoSortedArrays
{
    // Assume we placed arrays one under another, the shorter array is on top:
    //       [y]|[y,y]
    //   [x,x,x]|[x,x]
    //
    // The line in the middle is the cut line drawn to make sure number of the elements to the left of the line equal to (n + m) / 2
    // At the same time the number elements to the right of the line is also (n + m) / 2 if (n + m) is even or (n + m) / 2 + 1 otherwise.
    // We'll use binary search on the shorter array to move the cut.
    // Assume elements around the cut are labeled as follows
    //    ul | ur    // upper left & upper right
    //    bl | br    // bottom left & bottom right
    // Depending on the input data and current cut position each of the elements above may be either a number or undefined.
    // In case elements to the left of the cut are undefined we'll use int.MinValue instead.
    // Similarly if elements to the right to the cut are undefined we'll use int.MaxValue.
    // With the assumptions above the binary search completion criteria is 
    //    max(ul, bl) <= min(ur, br)
    // and the median is
    // a)    (max(ul, bl) + min(ur, br)) / 2  if total number of elements is even
    // b)    min(ur, br) otherwise because extra element of the array where length is odd always lays to the right of the cut
    //
    // On every step of the binary search we have to decide a direction of the next step
    // if ul > ur we should move upper array to the right and correspondingly bottom array to the left to keep balance
    // else if ur < bl we should move upper array to the left and the bottom array to the rigth.
    //
    // NOTE: we run binary searsh over shorter array to prevent getting cut index of other array out of bounds as that array is longer and
    // so should have enough elemens to cover any move of the shorter array
    // NOTE: cut index points to the first element to the right of the cut line
    // NOTE: cut index possible values are 0 <= i <= n ; where n is the length of the array.
    //       if i == n then all elements of the array lie to the left of the cut line
    public class Solution
    {
        public double FindMedianSortedArrays(int[] nums1, int[] nums2)
        {
            if ((nums1.Length, nums2.Length) is var (len1, len2) && len1 > len2) {
                (nums1, len1, nums2, len2) = (nums2, len2, nums1, len1);
            }
            var len = len1 + len2;
            var halfLen = len >> 1;
            var searchLen = len1 + 1; // We assume there is one virtual trailing place in the array for the search purposes
            var cut1 = searchLen >> 1;

            while (true) {
                var cut2 = halfLen - cut1;
                var ul = cut1 > 0 ? nums1[cut1 - 1] : int.MinValue;
                var ur = cut1 < len1 ? nums1[cut1] : int.MaxValue;
                var bl = cut2 > 0 ? nums2[cut2 - 1] : int.MinValue;
                var br = cut2 < len2 ? nums2[cut2] : int.MaxValue;

                if (ul > br) {
                    searchLen >>= 1;                      // Divide search len by 2 and use left part of old search interval
                    cut1 -= searchLen - (searchLen >> 1); // Move cut1 to the middle of new search interval by subtracting the size of its right half
                }
                else if (ur < bl) {
                    searchLen -= searchLen >> 1;          // Divide search len by 2 and use rigth part of old search interval
                    cut1 += searchLen >> 1;               // Move cut1 to the middle of new search interval by adding the size of its left half
                }
                else {
                    return (len & 1) > 0 ? Math.Min(ur, br) : (Math.Max(ul, bl) + Math.Min(ur, br)) / 2d;
                }
            }
        }
    }

    [Theory]
    [InlineData(new int[0], new[] { 2 }, 2.0d)]
    [InlineData(new[] { 2 }, new int[0], 2.0d)]
    [InlineData(new[] { 2 }, new[] { 2 }, 2.0d)]
    [InlineData(new int[0], new[] { 2, 2 }, 2.0d)]
    [InlineData(new[] { 1, 3 }, new[] { 2 }, 2.0d)]
    [InlineData(new[] { 1, 2 }, new[] { 3, 4 }, 2.5d)]
    [InlineData(new[] { 1, 4 }, new[] { 2, 3 }, 2.5d)]
    [InlineData(new[] { 3, 3 }, new[] { 1, 1, 1 }, 1.0d)]
    [InlineData(new[] { 3, 3, 3, 3, 3, 3, 3, 3 }, new[] { 1, 1, 1, 1, 1 }, 3.0d)]
    [InlineData(new[] { 3, 3, 3, 3, 3, 3, 3, 3 }, new[] { 1, 1, 1, 1, 1, 1 }, 3.0d)]
    [InlineData(new[] { 3, 3, 3, 3, 3, 3, 3, 3 }, new[] { 1, 1, 1, 1, 1, 1, 1 }, 3.0d)]
    [InlineData(new[] { 1, 1, 1, 1, 1, 1, 1, 1 }, new[] { 3, 3, 3, 3, 3 }, 1.0d)]
    [InlineData(new[] { 1, 1, 1, 1, 1, 1, 1, 1 }, new[] { 3, 3, 3, 3, 3, 3 }, 1.0d)]
    [InlineData(new[] { 1, 1, 1, 1, 1, 1, 1, 1 }, new[] { 3, 3, 3, 3, 3, 3, 3 }, 1.0d)]
    [InlineData(new[] { 1, 10 }, new[] { 2, 3, 4, 5, 6, 6, 7, 7, 8, 8, 9, 9 }, 6.5d)]
    [InlineData(new[] { 1, 2, 3, 5, 6 }, new[] { 4 }, 3.5d)]
    public void Solve(int[] nums1, int[] nums2, double expected)
    {
        Assert.Equal(expected, new Solution().FindMedianSortedArrays(nums1, nums2), 3);
    }

}