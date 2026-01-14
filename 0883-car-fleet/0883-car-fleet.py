from typing import List

class Solution:
    def carFleet(self, target: int, position: List[int], speed: List[int]) -> int:
        stack = []
        for p, s in sorted(zip(position, speed), reverse=True):
            print(p, s)
            time = (target - p) / s
            if stack and time <= stack[-1]:
                continue  
            stack.append(time) 
        
        return len(stack)
