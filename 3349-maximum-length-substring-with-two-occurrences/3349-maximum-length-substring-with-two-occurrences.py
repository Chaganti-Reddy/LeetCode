class Solution:
    def maximumLengthSubstring(self, s: str) -> int:
        maxLen, i = 0, 0
        seen = [0] * 26

        for j in range(len(s)):
            seen[ord(s[j]) - ord("a")] += 1

            while seen[ord(s[j]) - ord("a")] > 2:
                seen[ord(s[i]) - ord("a")] -= 1
                i += 1

            maxLen = max(maxLen, j - i + 1)

        return maxLen
