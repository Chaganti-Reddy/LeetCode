#include <stdio.h>
#include <stdlib.h>

#define MAX_UNIQUE 1000  // adjust if needed

typedef struct {
    int key;
    int count;
} MapItem;

int findKey(MapItem* map, int size, int key) {
    for (int i = 0; i < size; i++) {
        if (map[i].key == key) return i;
    }
    return -1;
}

int numRabbits(int* answers, int answersSize) {
    MapItem map[MAX_UNIQUE];
    int mapSize = 0;
    int result = 0;

    // Count frequency of each unique answer
    for (int i = 0; i < answersSize; i++) {
        int idx = findKey(map, mapSize, answers[i]);
        if (idx == -1) {
            map[mapSize].key = answers[i];
            map[mapSize].count = 1;
            mapSize++;
        } else {
            map[idx].count++;
        }
    }

    // Calculate minimum rabbits needed
    for (int i = 0; i < mapSize; i++) {
        int answer = map[i].key;
        int count = map[i].count;
        int groupSize = answer + 1;
        int groups = (count + groupSize - 1) / groupSize;  // ceil
        result += groups * groupSize;
    }

    return result;
}