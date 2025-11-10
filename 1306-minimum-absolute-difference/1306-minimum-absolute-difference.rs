impl Solution {
    pub fn minimum_abs_difference(mut arr: Vec<i32>) -> Vec<Vec<i32>> {
        let mut minDiff = i32::MAX;
        let mut ans = Vec::new();
        arr.sort_unstable();
        for i in 0..arr.len()-1{
            minDiff = minDiff.min(arr[i+1] - arr[i]);
        }
        for i in 0..arr.len()-1 {
            if(arr[i+1] - arr[i] == minDiff){
                ans.push(vec![arr[i], arr[i+1]]);
            }
        }
        ans
    }
}