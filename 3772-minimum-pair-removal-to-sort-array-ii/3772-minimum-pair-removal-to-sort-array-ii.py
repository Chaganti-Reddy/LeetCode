import heapq

class Solution:
    def minimumPairRemoval(self, nums: List[int]) -> int:
        n = len(nums)
        if n < 2: return 0
        
        R = [i + 1 for i in range(n)]
        L = [i - 1 for i in range(n)]
        R[n-1] = -1
        
        deleted = [False] * n
        minPairSet = []
        badPairs = 0

        def is_bad(i, j):
            if i == -1 or j == -1: return False
            return nums[i] > nums[j]

        for i in range(n - 1):
            if is_bad(i, i + 1):
                badPairs += 1
            heapq.heappush(minPairSet, (nums[i] + nums[i+1], i, i + 1))

        operations = 0
        while badPairs > 0 and minPairSet:
            s, i, j = heapq.heappop(minPairSet)

            if deleted[i] or deleted[j] or R[i] != j or s != nums[i] + nums[j]:
                continue

            left_of_i = L[i]
            right_of_j = R[j]

            if is_bad(left_of_i, i): badPairs -= 1
            if is_bad(i, j): badPairs -= 1
            if is_bad(j, right_of_j): badPairs -= 1

            nums[i] = nums[i] + nums[j]
            deleted[j] = True
            
            R[i] = right_of_j
            if right_of_j != -1:
                L[right_of_j] = i
            
            if is_bad(left_of_i, i): badPairs += 1
            if is_bad(i, right_of_j): badPairs += 1

            if left_of_i != -1:
                heapq.heappush(minPairSet, (nums[left_of_i] + nums[i], left_of_i, i))
            if right_of_j != -1:
                heapq.heappush(minPairSet, (nums[i] + nums[right_of_j], i, right_of_j))

            operations += 1
        
        return operations