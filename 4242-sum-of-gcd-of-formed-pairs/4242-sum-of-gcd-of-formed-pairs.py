import math

class Solution:
    def gcdSum(self, nums: list[int]) -> int:
        prefix = []
        mx = 0

        for x in nums:
            mx = max(mx, x)
            prefix.append(math.gcd(x, mx))

        prefix.sort()

        ans = 0
        l, r = 0, len(prefix) - 1

        while l < r:
            ans += math.gcd(prefix[l], prefix[r])
            l += 1
            r -= 1

        return ans