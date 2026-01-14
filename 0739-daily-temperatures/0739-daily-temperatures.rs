impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut stack: Vec<usize> = Vec::new();
        let n: usize = temperatures.len();
        let mut result: Vec<i32> = vec![0; n];

        for (i, t) in temperatures.iter().enumerate(){
            while !stack.is_empty() && temperatures[*stack.last().unwrap()] < *t{
                result[*stack.last().unwrap()] = (i - *stack.last().unwrap()) as i32;
                stack.pop();
            }
            stack.push(i);
        }
        result
    }
}