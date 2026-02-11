class Solution:
    def findMaxConsecutiveOnes(self, nums: List[int]) -> int:
        ans = 0
        m = 0
        for num in nums:
            if num == 1:
                m += 1
            else:
                ans = max(ans, m)
                m = 0
        
        return max(ans, m)