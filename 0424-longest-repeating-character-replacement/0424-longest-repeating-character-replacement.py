class Solution:
    def characterReplacement(self, s: str, k: int) -> int:
        count = {}  # Dictionary to store character frequencies
        max_freq = 0  # Frequency of the most common character in the window
        l = 0  # Left pointer of the window
        ans = 0  # Stores the max valid window length

        # Iterate through the string with the right pointer `r`
        for r in range(len(s)):
            count[s[r]] = count.get(s[r], 0) + 1  # Update count of s[r]
            max_freq = max(max_freq, count[s[r]])  # Track max frequency character

            # Check if the window is valid
            while (r - l + 1) - max_freq > k:
                count[s[l]] -= 1  # Decrease count of character at `l`
                l += 1  # Shrink the window from the left

            ans = max(ans, r - l + 1)  # Update the maximum valid window length

        return ans  # Return the longest valid substring length
