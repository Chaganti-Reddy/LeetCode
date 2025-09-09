class Solution {
private:
    void backtrack(vector<int>& nums, vector<vector<int>>& ans, int pos) {
        if (pos == nums.size()) {
            ans.push_back(nums);
            return;
        }

        unordered_set<int> used; 
        for (int i = pos; i < nums.size(); i++) {
            if (used.find(nums[i]) != used.end()) continue; 
            used.insert(nums[i]);
            swap(nums[pos], nums[i]);
            backtrack(nums, ans, pos + 1);
            swap(nums[pos], nums[i]);
        }
    }

public:
    vector<vector<int>> permuteUnique(vector<int>& nums) {
        vector<vector<int>> ans; 
        sort(nums.begin(), nums.end()); 
        backtrack(nums, ans, 0);       
        return ans;
    }
};
