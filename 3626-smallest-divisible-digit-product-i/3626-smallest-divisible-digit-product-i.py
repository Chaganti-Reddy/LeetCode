class Solution:
    def smallestNumber(self, n: int, t: int) -> int:
        def valid(num):
            prod = 1
            while num:
                digit = num % 10
                if digit == 0:
                    return True

                prod *= digit
                if prod % t == 0:
                    return True

                num //= 10

            return False

        while not valid(n):
            n += 1

        return n
