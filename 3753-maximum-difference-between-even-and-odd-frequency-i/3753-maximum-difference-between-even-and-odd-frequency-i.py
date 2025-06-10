class Solution:
    def maxDifference(self, s: str) -> int:
        arr = [0]*26
        odd, even = 0, 1000
        for c in s:
            arr[ord(c) - ord('a')] += 1
        
        for i in arr:
            if(i % 2 == 0 and i != 0):
                even = min(even, i)
            else:
                odd = max(odd, i)
                
        return odd - even