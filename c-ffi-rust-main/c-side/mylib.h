typedef struct Point {
    int x;
    int y;
} Point;

typedef struct Vector {
    int x;
    int y;
} Vector;

Vector* make_vector_ptrs(Point* start, Point* end);
const Vector* make_vector_const_ptr(const Vector* input);
Vector make_vector(Point start, Point end);
Vector sum_vectors(const Vector vecs[], size_t vecs_len);