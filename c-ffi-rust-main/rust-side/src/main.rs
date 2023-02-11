mod bindings;

fn main() {
    let mut start = bindings::Point {
        x: 1,
        y: 2,
    };

    let mut end = bindings::Point {
        x: 3,
        y: 5
    };

    let res = bindings::make_vector_ptrs(&mut start, &mut end);
    println!("make_vector_ptrs: {:?}", res);

    if let Some(vector) = res {
        let res2 = bindings::make_vector_const_ptr(vector);
        println!("make_vector_const_ptr: {:?}", res2);
    }

    let res3 = bindings::make_vector(start, end);
    println!("make_vector: {:?}", res3);

    let res4 = bindings::sum_vectors(&vec![
        bindings::Vector { x: 1, y: 2 },
        bindings::Vector { x: 3, y: 5 },
        bindings::Vector { x: 8, y: 1 }
    ]);
    println!("sum_vectors: {:?}", res4);

}