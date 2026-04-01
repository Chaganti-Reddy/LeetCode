impl Solution {
    pub fn survived_robots_healths(
        positions: Vec<i32>,
        healths: Vec<i32>,
        directions: String,
    ) -> Vec<i32> {
        let n = positions.len();
        let dirs: Vec<char> = directions.chars().collect();
        let mut indices: Vec<usize> = (0..n).collect();
        indices.sort_by_key(|&i| positions[i]);
        let mut healths = healths;
        let mut stack: Vec<usize> = Vec::new();
        
        for &i in &indices {
            if dirs[i] == 'R' {
                stack.push(i);
            } else {
                let mut alive = true;
                while let Some(&top) = stack.last() {
                    if dirs[top] == 'R' {
                        if healths[top] > healths[i] {
                            healths[top] -= 1;
                            alive = false;
                            break;
                        } else if healths[top] < healths[i] {
                            healths[i] -= 1;
                            stack.pop();
                        } else {
                            stack.pop();
                            alive = false;
                            break;
                        }
                    } else {
                        break;
                    }
                }
                if alive {
                    stack.push(i);
                }
            }
        }
        
        let mut survivors: Vec<(usize, i32)> = stack
            .iter()
            .map(|&i| (i, healths[i]))
            .collect();
        
        survivors.sort_by_key(|&(i, _)| i);
        survivors.into_iter().map(|(_, h)| h).collect()
    }
}