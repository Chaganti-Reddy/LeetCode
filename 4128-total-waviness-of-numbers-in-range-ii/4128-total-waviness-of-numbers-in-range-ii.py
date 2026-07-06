from functools import lru_cache

class Solution:
    def totalWaviness(self, num1: int, num2: int) -> int:

        def calculate(num: int) -> int:
            if num < 0:
                return 0

            s = str(num)
            n = len(s)

            @lru_cache(None)
            def score(pos, prevPrev, prev, tight, leadingZero):
                if pos == n:
                    return (1, 0)

                digitLimit = int(s[pos]) if tight else 9

                totalCount = 0
                totalScore = 0

                for digit in range(digitLimit + 1):
                    newTight = tight and (digit == digitLimit)
                    newLeadingZero = leadingZero and (digit == 0)

                    if newLeadingZero:
                        cnt, scr = score(
                            pos + 1,
                            -1,
                            -1,
                            newTight,
                            True
                        )
                    else:
                        if leadingZero:
                            # first non-leading digit
                            cnt, scr = score(
                                pos + 1,
                                -1,
                                digit,
                                newTight,
                                False
                            )
                        elif prevPrev == -1:
                            # second significant digit
                            cnt, scr = score(
                                pos + 1,
                                prev,
                                digit,
                                newTight,
                                False
                            )
                        else:
                            # check if prev is peak or valley
                            add = 1 if (
                                (prevPrev < prev > digit) or
                                (prevPrev > prev < digit)
                            ) else 0

                            cnt, scr = score(
                                pos + 1,
                                prev,
                                digit,
                                newTight,
                                False
                            )

                            scr += add * cnt

                    totalCount += cnt
                    totalScore += scr

                return (totalCount, totalScore)

            return score(0, -1, -1, True, True)[1]

        return calculate(num2) - calculate(num1 - 1)