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
    pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode{val: 0, next: head});
        let mut current = &mut dummy;

        while let Some(ref mut node) = current.next {
            if(node.val == val){
                current.next = node.next.take();
            } else {
                current = current.next.as_mut().unwrap();
            }
        }
        dummy.next
    }
}