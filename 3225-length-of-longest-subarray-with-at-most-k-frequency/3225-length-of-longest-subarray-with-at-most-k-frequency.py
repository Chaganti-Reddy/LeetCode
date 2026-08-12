class Solution:
    def maxSubarrayLength(self, nums: List[int], k: int) -> int:
        i = 0
        frequency = {}
        maxLength = 0

        for j in range(len(nums)):
            frequency[nums[j]] = frequency.get(nums[j], 0) + 1

            while frequency[nums[j]] > k:
                frequency[nums[i]] -= 1
                i += 1

            maxLength = max(maxLength, j - i + 1)

        return maxLength
