from collections import deque
class Solution:
    def maxSlidingWindow(self, nums: List[int], k: int) -> List[int]:
        if not nums:
            return []
        
        dq = deque()  # Stores indices of elements
        result = []
        
        for i in range(len(nums)):
            # Remove elements from front if they are out of the window
            if dq and dq[0] < i - k + 1:
                dq.popleft()
            
            # Remove elements from back that are smaller than the current element
            while dq and nums[dq[-1]] < nums[i]:
                dq.pop()
            
            # Add current element index
            dq.append(i)
            
            # Start adding max elements to result once the first window is formed
            if i >= k - 1:
                result.append(nums[dq[0]])  # Front of deque is the max element
            
        return result