# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    def mergeKLists(self, lists: List[Optional[ListNode]]) -> Optional[ListNode]:
        l = []
        for list in lists:
            while list:
                l.append(list.val)
                list = list.next
        dummy = ListNode(0)
        curr = dummy
        for val in sorted(l):
            dummy.next = ListNode(val)
            dummy = dummy.next
        return curr.next