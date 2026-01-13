class Solution:
    def isValid(self, s: str) -> bool:
        m = {
            ')': '(',
            '}': '{',
            ']': '['
        }
        stack = []

        for c in s:
            if c in m.values():
                stack.append(c)
            else:
                if not stack or stack.pop() != m[c]:
                    return False

        return not stack
