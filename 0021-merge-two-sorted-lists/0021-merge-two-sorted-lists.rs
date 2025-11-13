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
    pub fn merge_two_lists(
        mut list1: Option<Box<ListNode>>,
        mut list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {

        let mut dummy = Box::new(ListNode { val: 0, next: None });
        let mut tail = &mut dummy;

        while list1.is_some() && list2.is_some() {
            let l1 = list1.as_mut().unwrap();
            let l2 = list2.as_mut().unwrap();

            if l1.val <= l2.val {
                let next = l1.next.take();
                tail.next = list1.take();
                list1 = next;
            } else {
                let next = l2.next.take();
                tail.next = list2.take();
                list2 = next;
            }

            tail = tail.next.as_mut().unwrap();
        }

        tail.next = if list1.is_some() { list1 } else { list2 };

        dummy.next
    }
}
