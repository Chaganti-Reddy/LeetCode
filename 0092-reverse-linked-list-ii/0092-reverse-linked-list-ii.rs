// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>,
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode { next: None, val }
//   }
// }

impl Solution {
    pub fn reverse_between(
        head: Option<Box<ListNode>>,
        left: i32,
        right: i32
    ) -> Option<Box<ListNode>> {
        // Create a dummy node to simplify edge cases
        let mut dummy = Box::new(ListNode { val: 0, next: head });

        // `prev` will stop at (left - 1)-th node
        let mut prev = &mut dummy;
        for _ in 0..(left - 1) {
            prev = prev.next.as_mut().unwrap();
        }

        // Detach the sublist to reverse
        let mut curr = prev.next.take(); // now prev.next = None
        let mut prev_rev: Option<Box<ListNode>> = None;

        // Reverse nodes between left and right
        for _ in left..=right {
            if let Some(mut node) = curr {
                let next = node.next.take(); // detach node.next
                node.next = prev_rev;        // link to previous reversed part
                prev_rev = Some(node);       // move head of reversed list
                curr = next;                 // move to next node
            }
        }

        // Connect reversed part back into list
        prev.next = prev_rev; // connect left-1 â reversed_head

        // Move to the tail of the reversed segment
        let mut tail = prev;
        while let Some(ref mut next_node) = tail.next {
            tail = next_node;
        }

        // Connect tail of reversed part to remaining list
        tail.next = curr; // connect reversed_tail â right+1

        // Return new head (skip dummy)
        dummy.next
    }
}
