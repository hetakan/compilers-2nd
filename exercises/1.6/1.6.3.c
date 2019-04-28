#include <stdio.h>

void main() {
    {   int w, x, y, z;      /* ブロック B1 */ // w(B1-B3-B4), x(B1-B2-B3-B4), y(B1-B5), z(B1-B2-B5)
        {   int x, z;        /* ブロック B2 */ // x(B2-B3) z(B2)
            {   int w, x;    /* ブロック B3 */ } // w(B3), x(B3)
        }
        {   int w, x;        /* ブロック B4 */ // w(B4) x(B4)
            {   int y, z;    /* ブロック B5 */ } // y(B5) z(B5)
        }
    }
}