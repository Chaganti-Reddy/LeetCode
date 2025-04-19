from bisect import bisect_left, bisect_right

class Solution:
    def countFairPairs(self, nums: List[int], lower: int, upper: int) -> int:
        nums.sort()
        n = len(nums)
        ans = 0

        for i in range(n):
            l = bisect_left(nums, lower - nums[i], i + 1, n)
            r = bisect_right(nums, upper - nums[i], i + 1, n)
            ans += (r - l)

        return ans
