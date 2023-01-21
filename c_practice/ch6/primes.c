#include <stdio.h>
#include <math.h>

int GetInt(char* phrase) {
    printf("%s", phrase);
    int num;
    scanf("%d", &num);
    return num;
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

int main(void) {
    CheckPrime();
}