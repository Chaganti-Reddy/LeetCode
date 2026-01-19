use std::collections::BinaryHeap;
use std::cmp::Reverse;

struct KthLargest {
    k: usize,
    // We store Reverse<i32> to turn the Max-Heap into a Min-Heap
    heap: BinaryHeap<Reverse<i32>>,
}

impl KthLargest {
    fn new(k: i32, nums: Vec<i32>) -> Self {
        let k_usize = k as usize;
        let mut heap = BinaryHeap::new();

        for n in nums {
            heap.push(Reverse(n));
        }

        while heap.len() > k_usize {
            heap.pop();
        }

        KthLargest { k: k_usize, heap }
    }

    fn add(&mut self, val: i32) -> i32 {
        self.heap.push(Reverse(val));

        if self.heap.len() > self.k {
            self.heap.pop();
        }
        self.heap.peek().unwrap().0
    }
}

/**
 * Your KthLargest object will be instantiated and called as such:
 * let obj = KthLargest::new(k, nums);
 * let ret_1: i32 = obj.add(val);
 */