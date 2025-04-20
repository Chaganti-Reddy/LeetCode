class Solution:
    def numRabbits(self, answers: List[int]) -> int:
        m = Counter(answers)
        ans = 0
        for i in m:
            ans += ceil(float(m[i])/(i+1))*(i + 1)

        return ans 