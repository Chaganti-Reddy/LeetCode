class Solution {
private:
    bool isSquare(int num) {
        int r = sqrt(num);
        return r * r == num;
    }

    int dfs(vector<int>& nums, vector<bool>& used, int prev, int depth) {
        if (depth == nums.size()) return 1;

        int count = 0;
        for (int i = 0; i < nums.size(); i++) {
            if (used[i]) continue;

            if (i > 0 && nums[i] == nums[i-1] && !used[i-1]) continue;

            if (prev == -1 || isSquare(nums[prev] + nums[i])) {
                used[i] = true;
                count += dfs(nums, used, i, depth + 1);
                used[i] = false;
            }
        }
        return count;
    }

public:
    int numSquarefulPerms(vector<int>& nums) {
        sort(nums.begin(), nums.end()); 
        vector<bool> used(nums.size(), false);
        return dfs(nums, used, -1, 0);
    }
};
