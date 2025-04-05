from typing import List

class Solution:
    def search(self, nums: List[int], target: int) -> int:
        def binary_search(low, end):
            if low > end:
                return -1 
            mid = (low + end) // 2
            if nums[mid] == target:
                return mid
            elif nums[mid] > target:
                return binary_search(low, mid - 1)
            else:
                return binary_search(mid + 1, end)
        
        return binary_search(0, len(nums) - 1)
