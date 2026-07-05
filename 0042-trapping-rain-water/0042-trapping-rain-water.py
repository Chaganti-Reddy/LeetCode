class Solution:
    def trap(self, height: List[int]) -> int:
        if(len(height) == 0):
            return 0
        
        l, r = 0, len(height) - 1
        maxL, maxR, ans = height[l], height[r], 0

        while l < r:
            if height[l] <= height[r]:
                l += 1
                maxL = max(maxL, height[l])
                ans += maxL - height[l]

            else:
                r -= 1
                maxR = max(maxR, height[r])
                ans += maxR - height[r]

        return ans