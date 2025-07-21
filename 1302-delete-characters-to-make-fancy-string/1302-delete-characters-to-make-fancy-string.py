class Solution:
    def makeFancyString(self, s: str) -> str:
        count = 1
        ans = [s[0]] 
        
        for i in range(1, len(s)):
            if s[i] == s[i - 1] and count < 2:
                ans.append(s[i])
                count += 1
            elif s[i] != s[i - 1]:
                ans.append(s[i])
                count = 1 
        
        return ''.join(ans)  
