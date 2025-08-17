class Solution:
    def generateParenthesis(self, n: int) -> List[str]:
        valid = []

        def generate(s, open, close):
            if open == 0 and close == 0:
                valid.append("".join(s))
                return
            
            if open>0:
                s.append('(')
                generate(s, open-1, close)
                s.pop()
            if close>0:
                if open < close:
                    s.append(')')
                    generate(s, open, close-1)
                    s.pop()
        
        generate([], n, n)
        return valid