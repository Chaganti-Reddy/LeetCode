class Solution:
    def minBitwiseArray(self, nums: List[int]) -> List[int]:
        ans = [-1] * len(nums)
        for i, num in enumerate(nums):
            if num == 2:
                continue
            for j in range(0, 32):
                if (num & (1 << j)) > 0 :
                    continue

                prev =  j - 1
                x = (num ^ (1 << (j-1)))
                ans[i] = x
                break 
        
        return ans
            
