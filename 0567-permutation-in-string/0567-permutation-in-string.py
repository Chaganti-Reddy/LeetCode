class Solution:
    def checkInclusion(self, s1: str, s2: str) -> bool:
        if len(s1) > len(s2):
            return False

        freq = [0] * 26
        for c in s1:
            freq[ord(c) - 97] += 1

        needed = len(s1)
        left = 0

        for right in range(len(s2)):
            r = ord(s2[right]) - 97

            if freq[r] > 0:
                needed -= 1
            freq[r] -= 1

            if right - left + 1 > len(s1):
                l = ord(s2[left]) - 97
                if freq[l] >= 0:
                    needed += 1
                freq[l] += 1
                left += 1

            if needed == 0:
                return True

        return False
