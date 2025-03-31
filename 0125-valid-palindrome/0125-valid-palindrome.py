class Solution:
    def isPalindrome(self, s: str) -> bool:
        temp = re.sub(r'[^a-zA-Z0-9]', '', s)
        ans = temp.lower()
        return ans == ans[::-1]