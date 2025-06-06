#include <stdio.h>
#include <stdlib.h>
#include <string.h>

char* robotWithString(char* s) {
    int n = strlen(s);
    char* minSuffix = (char*)malloc(n);
    char* stack = (char*)malloc(n);
    char* result = (char*)malloc(n + 1);
    int top = 0, resIdx = 0;

    // Fill minSuffix array
    minSuffix[n - 1] = s[n - 1];
    for (int i = n - 2; i >= 0; --i)
        minSuffix[i] = s[i] < minSuffix[i + 1] ? s[i] : minSuffix[i + 1];

    for (int i = 0; i < n; ++i) {
        stack[top++] = s[i];
        while (top > 0 && (i == n - 1 || stack[top - 1] <= minSuffix[i + 1])) {
            result[resIdx++] = stack[--top];
        }
    }

    result[resIdx] = '\0';
    free(minSuffix);
    free(stack);
    return result;
}
