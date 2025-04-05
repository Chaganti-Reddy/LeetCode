from typing import List

class Solution:
    def search(self, nums: List[int], target: int) -> int:
        def binary(l, r):
            if l > r:
                return -1
            
            mid = (l + r) // 2
            
            if nums[mid] == target:
                return mid
            
            # Left half is sorted
            if nums[l] <= nums[mid]:
                if nums[l] <= target < nums[mid]:
                    return binary(l, mid - 1)
                else:
                    return binary(mid + 1, r)
            # Right half is sorted
            else:
                if nums[mid] < target <= nums[r]:
                    return binary(mid + 1, r)
                else:
                    return binary(l, mid - 1)
        
        return binary(0, len(nums) - 1)
