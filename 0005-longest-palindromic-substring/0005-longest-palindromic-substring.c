#include <stdio.h>
#include <stdlib.h>
#include <string.h>

char* expandAroundCenter(char* s, int left, int right) {
    int len = strlen(s);
    while (left >= 0 && right < len && s[left] == s[right]) {
        left--;
        right++;
    }

    int start = left + 1;
    int size = right - left - 1;

    char* substr = (char*)malloc(size + 1);
    strncpy(substr, s + start, size);
    substr[size] = '\0';

    return substr;
}

char* longestPalindrome(char* s) {
    int len = strlen(s);
    char* res = (char*)malloc(sizeof(char));
    res[0] = '\0';

    for (int i = 0; i < len; i++) {
        char* odd = expandAroundCenter(s, i, i);
        char* even = expandAroundCenter(s, i, i + 1);

        if (strlen(odd) > strlen(res)) {
            free(res);
            res = odd;
        } else {
            free(odd);
        }

        if (strlen(even) > strlen(res)) {
            free(res);
            res = even;
        } else {
            free(even);
        }
    }

    return res;
}
