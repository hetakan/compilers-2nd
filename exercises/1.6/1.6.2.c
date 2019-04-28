#include <stdio.h>

int main() {
    int w, x, y, z;
    int i = 3; int j = 4;
    {
        int i = 5;
        w = i + j;// 9(5+4)
    }
    x = i + j; // 7(3+4)
    {
        int j = 6;
        printf("y: %d\n", y);
        i = y;
        y = i + j; //?(?+6)
    }
    z = i + j; // ?(? + 4)
    printf("w: %d x: %d y: %d z: %d\n", w, x, y, z);
}