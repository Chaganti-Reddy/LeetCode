# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next

import heapq

class Solution:
    def mergeKLists(self, lists: List[Optional[ListNode]]) -> Optional[ListNode]:
        heap = []
        for l in lists:
            while l:
                heap.append(l.val)
                l = l.next
            
        heapq.heapify(heap)

        dummy = ListNode(0)
        temp = dummy
        while heap:
            temp.next = ListNode(heapq.heappop(heap))
            temp = temp.next
        
        return dummy.next