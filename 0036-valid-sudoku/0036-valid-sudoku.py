from typing import List

class Solution:
    def isValidSudoku(self, board: List[List[str]]) -> bool:
        # Check rows
        for i in range(len(board)):
            count = 0
            temp = set()
            for j in range(len(board[i])):
                if board[i][j] != ".":
                    count += 1
                    temp.add(board[i][j])
            
            if count != len(temp):
                return False

        # Check columns
        for i in range(len(board)):
            count = 0
            temp = set()
            for j in range(len(board[i])):
                if board[j][i] != ".":
                    count += 1
                    temp.add(board[j][i])
            
            if count != len(temp):
                return False
        
        # Check 3x3 sub-boxes
        for i in range(0, 9, 3):
            for j in range(0, 9, 3):
                count = 0
                temp = set()
                for k in range(3):
                    for l in range(3):
                        num = board[i + k][j + l]
                        if num != ".":
                            count += 1
                            temp.add(num)
                
                if count != len(temp):
                    return False
        
        return True