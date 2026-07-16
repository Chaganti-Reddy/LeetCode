class Solution:
    def gcdSum(self, nums: list[int]) -> int:
        n = len(nums)
        mx_i = 0
        prefixGcd = []

        for i in range(n):
            mx_i = max(mx_i, nums[i])
            prefixGcd.append(math.gcd(nums[i], mx_i))

        prefixGcd.sort()
        ans = 0

        for i in range(n//2):
            ans += math.gcd(prefixGcd[i], prefixGcd[n - i -1])

        return ans
