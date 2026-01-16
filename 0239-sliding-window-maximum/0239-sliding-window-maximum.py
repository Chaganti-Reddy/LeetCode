from collections import deque

class Solution:
    def maxSlidingWindow(self, nums: List[int], k: int) -> List[int]:
        queue = deque()
        result = []

        for i, n in enumerate(nums):
            while queue and nums[queue[-1]] < n:
                queue.pop()
            
            queue.append(i)
            
            if queue[0] == i - k:
                queue.popleft()

            if i >= k - 1:
                result.append(nums[queue[0]])
            
        return result