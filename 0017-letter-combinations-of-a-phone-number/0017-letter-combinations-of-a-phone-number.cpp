class Solution {
private:
    vector<string> ans; 
    
    void backtrack(string &digits, int index, string &path, vector<string> &mapping) {
        if (index == digits.size()) {      
            ans.push_back(path);
            return;
        }
        
        string letters = mapping[digits[index] - '0'];
        for (char c : letters) {
            path.push_back(c);             
            backtrack(digits, index + 1, path, mapping);
            path.pop_back();              
        }
    }
    
public:
    vector<string> letterCombinations(string digits) {
        if (digits.empty()) return {};
        
        vector<string> mapping = {
            "", "", "abc", "def", "ghi", "jkl", 
            "mno", "pqrs", "tuv", "wxyz"
        };
        
        string path;
        backtrack(digits, 0, path, mapping);
        return ans;
    }
};
