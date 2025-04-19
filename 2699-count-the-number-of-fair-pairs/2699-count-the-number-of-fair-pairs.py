from bisect import bisect_left, bisect_right

class Solution:
    def countFairPairs(self, nums: List[int], lower: int, upper: int) -> int:
        nums.sort()

        def count_common(l, u):
            return bisect_right(nums, u) - bisect_left(nums, l)

        ans = 0
        for num in nums:
            if (upper - num >= num and lower - num <= num):
                ans += (count_common(lower-num, upper-num) - 1)
            else:
                ans += count_common(lower-num, upper-num)

        return ans//2