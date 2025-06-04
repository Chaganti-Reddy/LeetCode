#define min(a,b) ((a) < (b) ? (a) : (b))
#define max(a,b) ((a) > (b) ? (a) : (b))

long long distributeCandies(int n, int limit) {
    long long ways =0 ;
    int minCh1 = max(0, n - 2*limit), maxCh1 = min(n, limit);
    for(int i = minCh1; i <= maxCh1; i++){
        ways += (min(n - i, limit) - max(0, n - i -  limit) + 1); 
    } 
    return ways; 
}