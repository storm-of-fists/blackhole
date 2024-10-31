#include <stdio.h>
#include "state.h"

#define FORMAT "%-14s %10.2f %10.2f %5.2f%%\n"

int main(void) {
    struct State alabama = {50750.0, 33945.0, "alabama"};
    struct State alaska = {591000.0, 201632.0, "alaska"};
    struct State arizona = {114000.0, 30287.0, "arizona"};

    struct State* states[] = {&alabama, &alaska, &arizona};
    
    for (int i = 0; i < sizeof(states) / sizeof(struct State*); i++) {
        struct State* st = states[i];
        printf(FORMAT, st->name, st->area, st->forest, get_percent(st));
    }
}