impl Solution {
    pub fn get_happy_string(n: i32, k: i32) -> String {
        let n = n as u32;
        let mut k = k - 1;
        
        let total_count = 3 * 2_i32.pow(n - 1);
        if k >= total_count {
            return "".to_string();
        }

        let mut res = String::new();
        let choices = vec!['a', 'b', 'c'];
        
        // 1. Determine first character
        let mut block_size = 2_i32.pow(n - 1);
        let first_idx = k / block_size;
        res.push(choices[first_idx as usize]);
        k %= block_size;

        // 2. Determine remaining characters
        for _ in 1..n {
            block_size /= 2;
            let prev = res.chars().last().unwrap();
            
            // Get the two options that aren't 'prev'
            let mut options = Vec::new();
            for &c in &choices {
                if c != prev {
                    options.push(c);
                }
            }

            let next_idx = k / block_size;
            res.push(options[next_idx as usize]);
            k %= block_size;
        }

        res
    }
}