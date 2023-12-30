#include <string.h>
#include <stdio.h>
#include <ctype.h>

#include "utility/utility.h"

#define MAX_LEN 100
#define DELIMITERS " "

int IsVowel(int ch) {
    int c = toupper(ch);
    return (c == 'A' || c == 'E' || c == 'I' || c == 'O' || c == 'U');
}

int FindFirstVowel(char* token) {
    for (int i = 0; i < strlen(token); i++) {
        if (IsVowel(token[i])) {
            return i;
        }
    }
    return -1;
}

int IsLegalWord(char* token) {
    // all strings end with some extra bs, strip it
    for (int i = 0; i < strlen(token) - 1; i++) {
        if (!isalpha(token[i])) {
            return 0;
        }
    }
    return 1;
}

void RearrangeWord(char* token, int vowelLocation) {
    if (vowelLocation < 0) {
        printf("%s ", token);
    }
    else if (vowelLocation == 0) {
        for (int i = 0; i < strlen(token) - 1; i++) {
            printf("%c", token[i]);
        }
        printf("way")
    }
    else {
        for (int i = vowelLocation; i < strlen(token) - 1; i++) {
            printf("%c", token[i]);
        }
        for (int i = 0; i < vowelLocation; i++) {
            printf("%c", token[i]);
        }
        printf("ay ");
    }
}

void TranslateLine(char* line) {
    char token[MAX_LEN];
    int firstVowelLocation;
    char* pch;
    pch = strtok(line, DELIMITERS);
    while (pch != NULL) {
        if (IsLegalWord(pch)) {
            firstVowelLocation = FindFirstVowel(pch);
            RearrangeWord(pch, firstVowelLocation);
        }
        else {
            printf("%s ", pch);
        }
        pch = strtok(NULL, DELIMITERS);
    }
}

int main(void) {
    char input[MAX_LEN];
    GetString("enter ur pig latin: ", input, MAX_LEN);
    TranslateLine(input);
    printf("\n");
}
