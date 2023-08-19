#include <stdio.h>

int GetInt(char* phrase) {
    printf("%s", phrase);
    int num;
    scanf("%d", &num);
    return num;
}

double CelsiusToFahrenheit(double c) {
    return (9.0 / 5.0 * c + 32);
}
