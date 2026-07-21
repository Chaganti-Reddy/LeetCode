class Solution:
    def maxActiveSectionsAfterTrade(self, s: str) -> int:
        n = len(s)
        i, ones, zeroes, prev, curr = 0, 0, 0, 0, 0

        while i < n and s[i] == '1':
            ones += 1
            i += 1
        
        while i < n and s[i] == '0':
            prev += 1
            i += 1
        
        while i < n:
            while i < n and s[i] == '1':
                ones += 1
                i += 1
            
            if (i==n):
                break
        
            while i < n and s[i] == '0':
                curr += 1
                i += 1
            
            zeroes = max(zeroes, prev + curr)
            prev = curr
            curr = 0
        
        return zeroes + ones


