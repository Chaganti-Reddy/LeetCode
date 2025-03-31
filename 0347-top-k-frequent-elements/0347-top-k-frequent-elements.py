from collections import Counter

class Solution:
    def topKFrequent(self, nums: List[int], k: int) -> List[int]:
        c = Counter(nums)
        sorted_c = sorted(c.items(), key = lambda x:-x[1])
        result = [i for i, freq in sorted_c[:k]]
        return result