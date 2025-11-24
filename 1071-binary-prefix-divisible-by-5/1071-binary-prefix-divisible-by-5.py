class Solution:
    def prefixesDivBy5(self, nums: List[int]) -> List[bool]:
        ans = []
        k = 0
        for num in nums:
            k = 2*k + num
            if k%5 == 0: ans.append(True)
            else: ans.append(False)
        
        return ans