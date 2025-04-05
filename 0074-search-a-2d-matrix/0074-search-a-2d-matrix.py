class Solution:
    def searchMatrix(self, matrix: List[List[int]], target: int) -> bool:
        def binary_search(low, end, row):
            if low > end:
                return False
            mid = (low + end) // 2
            if matrix[row][mid] == target:
                return True
            elif matrix[row][mid] > target:
                return binary_search(low, mid-1, row)
            else:
                return binary_search(mid + 1, end, row)

        for row in range(len(matrix)):
            if matrix[row][-1] < target:
                continue
            elif matrix[row][0] == target or matrix[row][-1] == target:
                return True
            else:
                return binary_search(0, len(matrix[row]), row)
            
        return False