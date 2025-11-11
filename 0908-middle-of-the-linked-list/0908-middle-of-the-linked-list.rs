// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn middle_node(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut slow = &head;
        let mut fast = &head;

        while let (Some(f), Some(next)) = (fast.as_ref(), fast.as_ref().and_then(|n| n.next.as_ref())) {
            fast = &next.next;
            slow = &slow.as_ref().unwrap().next;
        }

        slow.clone()
    }
}