int missingNumber(int* nums, int numsSize) {
    int t1 = 0, t2 = 0; 
    for(int i = 0; i<= numsSize; i++)
        t1 ^= i; 
    for(int i = 0; i <= numsSize-1; i++)
        t2 ^= nums[i]; 

    return t1^t2;    
}