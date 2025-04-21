/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     struct ListNode *next;
 * };
 */
struct ListNode* mergeTwoLists(struct ListNode* list1, struct ListNode* list2) {
    struct ListNode* dummy = malloc(sizeof(struct ListNode));
    dummy->val = -1;  
    dummy->next = NULL;
    struct ListNode* head = dummy;
    while(list1 && list2){
        if(list1->val <= list2->val){
            dummy->next = list1; 
            list1 = list1->next; 
        }
        else{
            dummy->next = list2; 
            list2 = list2->next; 
        }
        dummy = dummy->next; 
    }
    while(list1){
        dummy->next = list1; 
        list1 = list1->next;  
        dummy = dummy->next; 
    }
    while(list2){
        dummy->next = list2; 
        list2 = list2->next; 
        dummy = dummy->next; 
    }
    return head->next; 
}