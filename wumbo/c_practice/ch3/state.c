typedef struct State {
    float area;
    float forest;
    char name[10];
} State;

float get_percent(struct State *st) {
    return 100 * (st->forest / st->area);
}