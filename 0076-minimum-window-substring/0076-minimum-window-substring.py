from collections import Counter

class Solution:
    def minWindow(self, s: str, t: str) -> str:
        if not s or not t:
            return ""
        
        t_count = Counter(t)
        window = {}
        have, need = 0, len(t_count)
        res, res_len = "", float('inf')
        l = 0
        
        for r, c in enumerate(s):
            window[c] = window.get(c, 0) + 1
            
            if c in t_count and window[c] == t_count[c]:
                have += 1
            
            while have == need:
                if (r - l + 1) < res_len:
                    res = s[l:r+1]
                    res_len = r - l + 1
                
                window[s[l]] -= 1
                if s[l] in t_count and window[s[l]] < t_count[s[l]]:
                    have -= 1
                l += 1
        
        return res