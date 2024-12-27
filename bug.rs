fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    let mut iter = vec.iter();

    println!("First element: {}", iter.next().unwrap());
    println!("Second element: {}", iter.next().unwrap());

    // ERROR:  The iterator is exhausted.
    // Attempting to access elements after the iterator has been exhausted will panic.
    println!("Third element: {}", iter.next().unwrap());
}