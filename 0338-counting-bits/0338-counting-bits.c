/**
 * Note: The returned array must be malloced, assume caller calls free().
 */
#include <stdlib.h>

int setbits(int); 

int* countBits(int n, int* returnSize) {
    *returnSize = n + 1;  
    int* ans = (int*)malloc((n+1) * sizeof(int));
    for(int i = 0; i <= n; i++){
        ans[i] = setbits(i); 
    }
    return ans; 
}


int setbits(int num){
    int count = 0;
    while (num) {
        if (num & 1) count++;
        num = num >> 1;
    }
    return count;
}
