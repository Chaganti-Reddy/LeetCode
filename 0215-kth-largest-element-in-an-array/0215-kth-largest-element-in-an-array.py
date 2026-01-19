import heapq

class Solution:
    def findKthLargest(self, nums: List[int], k: int) -> int:
        heapq.heapify(nums)
        while nums and len(nums) > k:
            heapq.heappop(nums)

        return nums[0]