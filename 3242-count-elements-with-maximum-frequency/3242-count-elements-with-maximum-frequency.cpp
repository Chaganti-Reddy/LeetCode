class Solution {
public:
    int maxFrequencyElements(vector<int>& nums) {
        unordered_map<int, int> m; 
        int maxi = 0;

        for(int i=0;i<nums.size(); i++){
            m[nums[i]]++;
            if(maxi < m[nums[i]]) maxi = m[nums[i]];
        }
        int ans=0;
        for(auto it:m){
            if(it.second == maxi){
                ans += it.second; 
            }
        }
        return ans;
    }
};