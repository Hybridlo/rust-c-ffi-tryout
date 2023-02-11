use super::bindings;

pub fn make_vector_ptrs(start: &mut bindings::Point, end: &mut bindings::Point) -> Option<bindings::Vector> {
    unsafe {
        let res = bindings::make_vector_ptrs(start, end);

        if res.is_null() {
            None
        } else {
            Some(*res)
        }
    }
}

pub fn make_vector_const_ptr(vector_input: bindings::Vector) -> Option<bindings::Vector> {
    unsafe {
        let res = bindings::make_vector_const_ptr(&vector_input);

        if res.is_null() {
            None
        } else {
            Some(*res)
        }
    }
}

pub fn make_vector(start: bindings::Point, end: bindings::Point) -> bindings::Vector {
    unsafe {
        bindings::make_vector(start, end)
    }
}

pub fn sum_vectors(vecs: &[bindings::Vector]) -> bindings::Vector {
    unsafe {
        bindings::sum_vectors(vecs.as_ptr(), vecs.len())
    }
}