class Solution:
    def numberOfArrays(self, differences: List[int], lower: int, upper: int) -> int:
        max_val = min_val = cur = 0

        for i in differences:
            cur += i
            min_val = min(min_val, cur)
            max_val = max(max_val, cur)
        
        return max(0, (upper - lower) - (max_val - min_val) + 1)