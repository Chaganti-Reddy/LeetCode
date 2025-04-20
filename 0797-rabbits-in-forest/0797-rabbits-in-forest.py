class Solution:
    def numRabbits(self, answers: List[int]) -> int:
        answers.sort()
        ans = count = 0
        for i, v in enumerate(answers):
            if v == 0:
                ans += 1
            elif (i == 0 or v != answers[i-1] or count == 0):
                ans += v + 1
                count = v
            else:
                count -= 1

        return ans