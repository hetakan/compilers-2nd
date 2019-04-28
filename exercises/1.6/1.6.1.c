#include <stdio.h>

void main() {

    int w, x, y, z;
    int i = 4; int j = 5;
    { 
        int j = 7;
        i = 6;
        w = i + j; // 13(6+7)
    }
    x = i + j; // 11(6+5)
    { 
        int i = 8;
        y = i + j; // 13(8+5)
    }
    z = i + j; // 11(6+5)
    printf("w: %d x: %d y: %d z: %d\n", w, x, y, z);
}

