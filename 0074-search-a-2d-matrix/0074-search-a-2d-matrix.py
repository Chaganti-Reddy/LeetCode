from typing import List

class Solution:
    def searchMatrix(self, matrix: List[List[int]], target: int) -> bool:
        def binary_search(low, high, row):
            if low > high:
                return False
            mid = (low + high) // 2
            if matrix[row][mid] == target:
                return True
            elif matrix[row][mid] > target:
                return binary_search(low, mid - 1, row)
            else:
                return binary_search(mid + 1, high, row)

        for row in range(len(matrix)):
            if matrix[row][0] <= target <= matrix[row][-1]:
                if binary_search(0, len(matrix[row]) - 1, row):
                    return True
        return False
