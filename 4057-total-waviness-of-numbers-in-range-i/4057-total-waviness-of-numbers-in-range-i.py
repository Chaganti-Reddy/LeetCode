class Solution:
    def totalWaviness(self, num1: int, num2: int) -> int:
        if num2 <= 100:
            return 0

        ans = 0
        while num1 <= num2:
            if num1 <= 100:
                num1 += 1
                continue
            
            n = str(num1)
            i = 1
            while i < len(n) - 1:
                if ((n[i] > n[i-1] and n[i] > n[i+1]) or (n[i] < n[i-1] and n[i] < n[i+1])):
                    ans += 1
                
                i += 1

            num1 += 1
        
        return ans