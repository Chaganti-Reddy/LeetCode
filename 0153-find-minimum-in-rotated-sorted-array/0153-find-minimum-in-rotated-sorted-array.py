class Solution:
    def findMin(self, nums: List[int]) -> int:
        if nums[0] <= nums[-1]:
            return nums[0]
        else:
            l, r, minel = 0, len(nums) - 1, min(nums[0], nums[-1])
            while l <= r:
                mid = (l + r) // 2
                if minel < nums[mid]:
                    l = mid + 1
                else:
                    minel = min(minel, nums[mid])
                    r = mid - 1
                
        return minel