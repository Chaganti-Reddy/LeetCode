class Solution:
    def checkInclusion(self, s1: str, s2: str) -> bool:
        from collections import Counter
        
        len_s1, len_s2 = len(s1), len(s2)
        if len_s1 > len_s2:
            return False
        
        s1_count = Counter(s1)
        s2_count = Counter(s2[:len_s1])
        
        if s1_count == s2_count:
            return True
        
        for i in range(len_s1, len_s2):
            start_char, new_char = s2[i - len_s1], s2[i]
            
            s2_count[new_char] += 1
            s2_count[start_char] -= 1
            
            if s2_count[start_char] == 0:
                del s2_count[start_char]
            
            if s1_count == s2_count:
                return True
        
        return False