#include <stdio.h>

int GetInt(char* phrase) {
    printf("%s", phrase);
    int num;
    scanf("%d", &num);
    return num;
}

int main(void) {
    int n;
    n = GetInt("What is your card's rank? (1-13) ");
    switch(n) {
        case  1: printf("Ace\n"); break;
        case 11: printf("Jack\n"); break;
        case 12: printf("Queen\n"); break;
        case 13: printf("King\n"); break;
        default: printf("%i\n", n); break;
    }
}