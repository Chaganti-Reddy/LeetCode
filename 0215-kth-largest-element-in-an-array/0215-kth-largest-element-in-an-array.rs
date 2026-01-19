use std::collections::BinaryHeap;

impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, mut k: i32) -> i32 {
        let mut heap = BinaryHeap::new();
        for n in nums{
            heap.push(n);
        }
        
        while k - 1 > 0{
            k -= 1;
            heap.pop();
        }

        *heap.peek().unwrap()
    }
}