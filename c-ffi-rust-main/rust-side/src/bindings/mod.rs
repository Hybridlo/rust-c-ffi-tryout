mod bindings;
mod safe;

pub use bindings::{Point, Vector};
pub use safe::make_vector_ptrs;
pub use safe::make_vector_const_ptr;
pub use safe::make_vector;
pub use safe::sum_vectors;