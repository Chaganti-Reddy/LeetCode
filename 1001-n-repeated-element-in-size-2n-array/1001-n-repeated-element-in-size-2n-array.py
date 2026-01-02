class Solution:
    def repeatedNTimes(self, A: list[int]) -> int:
        for i in range(len(A) - 1):
            if A[i] == A[i + 1] or (i + 2 < len(A) and A[i] == A[i + 2]):
                return A[i]
        return A[0]