class Solution:
    def longestConsecutive(self, nums: List[int]) -> int:
        numSet = set(nums)
        longest = 0

        for num in numSet:
            if num - 1 not in numSet:
                curr = num
                streak = 1
                while curr + 1 in numSet:
                    curr += 1
                    streak += 1
                longest = max(longest, streak)

        return longest
