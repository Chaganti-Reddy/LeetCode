# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    def addTwoNumbers(self, l1: Optional[ListNode], l2: Optional[ListNode]) -> Optional[ListNode]:        
        list1 = l1
        list2 = l2
        carry = 0
        head = dummy = ListNode(-1)
        while list1 and list2:
            dummy.next = ListNode((list1.val+list2.val+carry)%10)
            dummy = dummy.next
            carry = (list1.val+list2.val+carry)//10
            list1 = list1.next
            list2 = list2.next
        
        while list1:
            dummy.next = ListNode((list1.val+carry)%10)
            dummy = dummy.next
            carry = (list1.val+carry)//10
            list1 = list1.next
         
        while list2:
            dummy.next = ListNode((list2.val+carry)%10)
            dummy = dummy.next
            carry = (list2.val+carry)//10
            list2 = list2.next  

        if carry > 0:
            dummy.next = ListNode(carry) 

        return head.next