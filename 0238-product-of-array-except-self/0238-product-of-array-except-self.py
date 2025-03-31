class Solution:
    def productExceptSelf(self, nums: List[int]) -> List[int]:
        output = [1]*len(nums)
        right = 1
        for i in range(len(nums)):
            output[i] *= right
            right *= nums[i]
        
        left = 1
        for i in range(len(nums)-1, -1, -1):
            output[i] *= left
            left *= nums[i]
        
        return output