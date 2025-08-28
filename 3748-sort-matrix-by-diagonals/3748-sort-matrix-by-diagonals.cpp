class Solution {
public:
    void sortDiagonal(int r, int c, vector<vector<int>>& grid, bool asc){
        int i=r, j=c, n=grid.size(); 
        vector<int> v; 
        while(i<n && j<n){
            v.push_back(grid[i][j]);
            i++; 
            j++;
        }
        if(asc){
            sort(begin(v), end(v)); 
        }
        else{
            sort(rbegin(v), rend(v));
        }
        i=r, j=c; 
        for(int val:v){
            grid[i][j] = val;
            i++; 
            j++;
        }
    }

    vector<vector<int>> sortMatrix(vector<vector<int>>& grid) {
        int n = grid.size(); 
        for(int row=0; row <n ; row++)
        {
            sortDiagonal(row, 0, grid, false); 
        }
        for(int col=1; col <n ; col++)
        {
            sortDiagonal(0, col, grid, true); 
        }
        return grid;
    }
};