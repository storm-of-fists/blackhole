#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#include <string.h>

#include "utility.h"

int FileManager(int (*f)(int)) {
    return f(3);
}


int GetInt(char* phrase) {
    printf("%s", phrase);
    int num;
    scanf("%d", &num);
    return num;
}

float GetReal(char* phrase) {
    printf("%s", phrase);
    float num;
    scanf("%f", &num);
    return num;
}

char* GetString(char* phrase, char* var, int max_len) {
    printf("%s", phrase);
    fgets(var, max_len, stdin);
    return var;
}

int IsPrime(int n) {
    int limit = sqrt(n) + 1;
    if (n <= 1) return 0;
    if (n == 2) return 1;
    if (n % 2 == 0) return 0;
    for (int i = 3; i < limit; i += 2) {
        if (n % i == 0) return 0;
    }
    return 1;
}

void CheckPrime(void) {
    int n = GetInt("Input number to check primacy: ");
    int prime = IsPrime(n);
    printf("Primacy of %d: %d\n", n, prime);
}

double CelsiusToFahrenheit(double c) {
    return (9.0 / 5.0 * c + 32);
}

typedef struct State {
    float area;
    float forest;
    char name[10];
} State;

float get_percent(struct State *st) {
    return 100 * (st->forest / st->area);
}

int RandomInteger(int low, int high) {
    int k;
    double d;

    d = (double) rand() / ((double) RAND_MAX + 1);
    k = (int) (d * (high - low + 1));
    return (low + k);
}

directionT OppositeDirection(directionT dir) {
    switch (dir) {
        case north: return (south);
        case east: return (west);
        case west: return (east);
        case south: return (north);
        default: perror("illegal direction value");
    }
}