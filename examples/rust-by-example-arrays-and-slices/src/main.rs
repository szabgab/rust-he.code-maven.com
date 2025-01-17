use std::mem;

// This function borrows a slice.
fn analyze_slice(slice: &[i32]) {
    println!("First element of the slice: {}", slice[0]);
    println!("The slice has {} elements", slice.len());
    //println!("First element of the slice: {}", slice[10]);
}

fn main() {
    // Fixed-size array (type signature is superfluous).
    let xs: [i32; 5] = [11, 21, 31, 41, 51];

    // All elements can be initialized to the same value.
    //let ys: [i8; 500] = [0; 500];
    let ys = [0; 500];
    //let ys = [0i8; 500];

    // Indexing starts at 0.
    println!("First element of the array: {}", xs[0]);
    println!("Second element of the array: {}", xs[1]);

    // `len` returns the count of elements in the array.
    println!("Number of elements in array: {}", xs.len());
    println!("Number of elements in array: {}", ys.len());

    // Arrays are stack allocated.
    println!("Array occupies {} bytes", mem::size_of_val(&xs));
    println!("Array occupies {} bytes", mem::size_of_val(&ys));

    // Arrays can be automatically borrowed as slices.
    println!("Borrow the whole array as a slice.");
    analyze_slice(&xs);

    // Slices can point to a section of an array.
    // They are of the form [starting_index..ending_index].
    // `starting_index` is the first position in the slice.
    // `ending_index` is one more than the last position in the slice.
    println!("Borrow a section of the array as a slice.");
    analyze_slice(&xs[1 .. 4]);
    analyze_slice(&ys[1 .. 20]);
    analyze_slice(&ys[1 ..=20]);

    // Example of empty slice `&[]`:
    let empty_array: [u32; 0] = [];
    assert_eq!(&empty_array, &[]);
    //println!("hello");
    assert_eq!(&empty_array, &[][..]); // Same but more verbose

    let one_array: [u32; 1] = [2];
    assert_eq!(&one_array, &[2]);
    //println!("hello");

    // Out of bound indexing on array with constant value causes compile time error.
    // println!("{}", xs[7]);
    // Out of bound indexing on slice causes runtime error.
    //println!("{}", xs[..][7]);


    // Arrays can be safely accessed using `.get`, which returns an
    // `Option`. This can be matched as shown below, or used with
    // `.expect()` if you would like the program to exit with a nice
    // message instead of happily continue

    let x = xs.get(0);
    println!("{:?}", x);

    for i in 0..xs.len() + 1 { // Oops, one element too far!
        match xs.get(i) {
            Some(xval) => println!("{}: {}", i, xval),
            None => println!("Slow down! {} is too far!", i),
        }
    }

}