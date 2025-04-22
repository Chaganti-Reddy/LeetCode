class Solution:
    def missingNumber(self, nums: List[int]) -> int:
        ans = 0
        for n in nums:
            ans ^= n
        tmp = 0
        for i in range(len(nums)+1):
            tmp ^= i
        return ans^tmp