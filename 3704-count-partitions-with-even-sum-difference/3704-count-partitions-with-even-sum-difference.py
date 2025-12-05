class Solution:
    def countPartitions(self, nums: List[int]) -> int:
        sum = ans = partition_sum = 0
        for n in nums:
            sum += n
        
        for i, n in enumerate(nums):
            partition_sum += n
            sum -= n
            if (partition_sum - sum) % 2 == 0 and i != len(nums) - 1:
                ans += 1

        return ans