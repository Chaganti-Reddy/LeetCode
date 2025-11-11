impl Solution {
    fn backtrack(n: i32, k: i32, ans: &mut Vec<Vec<i32>>, temp: &mut Vec<i32>, i: i32) {
        if temp.len() as i32 == k {
            ans.push(temp.clone());
            return;
        }

        if i > n {
            return;
        }

        temp.push(i);
        Self::backtrack(n, k, ans, temp, i + 1);
        temp.pop();

        Self::backtrack(n, k, ans, temp, i + 1);
    }

    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut ans = Vec::new();
        let mut temp = Vec::new();
        Self::backtrack(n, k, &mut ans, &mut temp, 1);
        ans
    }
}
