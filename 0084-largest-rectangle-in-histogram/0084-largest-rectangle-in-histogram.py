class Solution:
    def largestRectangleArea(self, heights: list[int]) -> int:
        # We add 0 at the end to force the stack to empty
        # This acts as a "Right Limit" for everything remaining
        heights.append(0)
        stack = []
        max_area = 0
    
        for i, current_h in enumerate(heights):
            # If current bar is smaller than stack top, calculate area for stack top
            while stack and heights[stack[-1]] >= current_h:
                h = heights[stack.pop()]
    
                # If stack is empty, it means h was the smallest so far.
                # It extends all the way from 0 to i.
                if not stack:
                    width = i
                else:
                    width = i - stack[-1] - 1
    
                max_area = max(max_area, h * width)
    
            stack.append(i)
    
        return max_area