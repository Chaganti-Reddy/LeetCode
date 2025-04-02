class Solution:
    def characterReplacement(self, s: str, k: int) -> int:
        count = {}
        max_freq = 0
        l = 0
        ans = 0

        for r in range(len(s)):
            count[s[r]] = count.get(s[r], 0) + 1
            max_freq = max(max_freq, count[s[r]])

            # If window is invalid, shrink it
            while (r - l + 1) - max_freq > k:
                count[s[l]] -= 1
                l += 1  # Shrink window

            ans = max(ans, r - l + 1)  # Update max length

        return ans
