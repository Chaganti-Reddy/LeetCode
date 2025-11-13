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
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        let mut slow = &head;
        let mut fast = &head;

        while fast.is_some() && fast.as_ref().unwrap().next.is_some() {
            slow = &slow.as_ref().unwrap().next;
            fast = &fast.as_ref().unwrap().next.as_ref().unwrap().next;
        }

        let mut prev: Option<Box<ListNode>> = None;
        let mut curr = slow.clone();

        while let Some(mut node) = curr {
            curr = node.next.take();
            node.next = prev;
            prev = Some(node);
        }

        let mut first_half = &head;
        let mut second_half = &prev;

        while second_half.is_some() {
            if(first_half.as_ref().unwrap().val != second_half.as_ref().unwrap().val) {
                return false;
            }
            first_half = &first_half.as_ref().unwrap().next;
            second_half = &second_half.as_ref().unwrap().next;
        }
        return true;
    }
}