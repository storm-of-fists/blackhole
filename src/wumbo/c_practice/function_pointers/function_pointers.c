#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>

int64_t add(int64_t, int64_t);
int64_t sub(int64_t, int64_t);
uint32_t max(uint32_t, uint32_t);

int main()
{
    // const char *str = "static";
    // printf("str: %p\n", str);
    // printf("main: %p\n", main);

    // unsigned int nice = 100;

    // printf("shift: %d\n", nice << 2);

    // int64_t (*f_ptr)(int64_t, int64_t);

    // f_ptr = *add;
    // printf("add : %ld\n", f_ptr(1, 2));
    // f_ptr = *sub;
    // f_ptr(1, 2);

    // uint8_t fkd_string[16] = {72, 101, 108, 108, 111, 44, 32, 87, 111, 114, 108, 100, 63, 33, 10, 0};

    // printf("fucked up string: %s\n", fkd_string);

    uint32_t i, h;
    i = 0;
    h = 50;
    if (max(i, h) == h)
    {
        printf("only in 2020\n");
    }
    i = 10;
    h = 0;
    if (max(i, h) != i)
    {
        printf("What even if life?\n");
    }
    return EXIT_SUCCESS;
}

int64_t add(int64_t a, int64_t b)
{
    return a + b;
}

int64_t sub(int64_t a, int64_t b)
{
    return a - b;
}

uint32_t max(uint32_t a, uint32_t b)
{
    uint32_t result;
    if (a > b)
    {
        result = a;
    }
    if (b > a)
    {
        result = b;
    }
    return result;
}