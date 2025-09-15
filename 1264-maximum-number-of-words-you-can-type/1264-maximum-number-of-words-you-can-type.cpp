class Solution {
public:
    int canBeTypedWords(string text, string brokenLetters) {
        unordered_set<char> broken(brokenLetters.begin(), brokenLetters.end());
        int ans = 0;
        bool brokenWord = false;

        for (int i = 0; i < text.size(); i++) {
            if (text[i] == ' ') {
                if (!brokenWord) ans++;
                brokenWord = false;       
            } 
            else if (broken.count(text[i])) {
                brokenWord = true;       
            }
        }

        if (!brokenWord) ans++;

        return ans;
    }
};
