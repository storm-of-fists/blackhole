#include <stdio.h>
#include <string.h>
#include <stdlib.h>

#include "utility/utility.h"

typedef enum {
    Sunday, Monday, Tuesday
} weekdayT;

int main(void) {
    directionT dir = south;
    directionT new_dir = OppositeDirection(dir);
    printf("new dir: %i\n", new_dir);
    printf("%i\n", (int) strlen("hello"));
    char str1[] = "bitch";
    char str2[] = "car";
    char* new_string = (char*) malloc((strlen(str1) + strlen(str2) + 1) * sizeof(char));
    strcpy(new_string, str1);
    strcat(new_string, str2);
    printf("%s", new_string);
}
