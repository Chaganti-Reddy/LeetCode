impl Solution {
    pub fn time_required_to_buy(tickets: Vec<i32>, k: i32) -> i32 {
        let mut ans = 0;
        for (i, x) in tickets.iter().enumerate() {
            if(i as i32 <= k) {
                ans += tickets[i].min(tickets[k as usize]);
            } else {
                ans += tickets[i].min(tickets[k as usize] - 1);
            }
        }
        ans
    }
}