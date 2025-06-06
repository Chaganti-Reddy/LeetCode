class Solution:
    def robotWithString(self, s: str) -> str:
        n = len(s)
        min_suffix = [''] * n
        min_suffix[-1] = s[-1]
        
        for i in range(n - 2, -1, -1):
            min_suffix[i] = min(s[i], min_suffix[i + 1])

        t = []
        res = []
        j = 0

        for i in range(n):
            t.append(s[i])
            while t and (j == n - 1 or t[-1] <= min_suffix[i + 1] if i + 1 < n else True):
                res.append(t.pop())

        return ''.join(res)
