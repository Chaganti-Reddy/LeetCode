class Solution:
    def twoSum(self, nums: List[int], target: int) -> List[int]:
        m = {}
        for i, n in enumerate(nums):
            d = target-n
            if d in m:
                return [i, m[d]]
            m[n] = i
        
        return 
