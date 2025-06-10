#define max(a, b) ((a) > (b) ? (a) : (b))
#define min(a, b) ((a) < (b) ? (a) : (b))

int maxDifference(char* s) {
    int arr[26], odd = 0, even = 1000;
    for(int i = 0; s[i]!='\0'; i++){
        arr[s[i] - 'a']++; 
    }
    for(int i = 0; i < 26; i++){
        if(arr[i] % 2 == 0 && arr[i] != 0){
            even = min(even, arr[i]);
        }
        else{
            odd = max(odd, arr[i]); 
        }
    }
    return odd - even;
}