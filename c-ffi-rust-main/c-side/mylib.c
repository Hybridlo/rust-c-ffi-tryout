#include "mylib.h"
#include <stdlib.h>

Vector* make_vector_ptrs(Point* start, Point* end) {
    Vector* res = malloc(sizeof(Point));

    res->x = end->x - start->x;
    res->y = end->y - start->y;

    return res;
}

const Vector* make_vector_const_ptr(const Vector* input) {
    const Vector* res = input;

    return res;
}

Vector make_vector(Point start, Point end) {
    Vector res = {end.x - start.x, end.y - start.y};

    return res;
}

Vector sum_vectors(const Vector vecs[], size_t vecs_len) {
    Vector res = {0, 0};

    for (size_t i = 0; i < vecs_len; i++) {
        res.x += vecs[i].x;
        res.y += vecs[i].y;
    }

    return res;
}