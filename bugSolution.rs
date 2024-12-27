fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    let mut iter = vec.iter();

    println!("First element: {}", iter.next().unwrap());
    println!("Second element: {}", iter.next().unwrap());

    // Check if there are any more elements before attempting to access them.
    if let Some(third) = iter.next() {
        println!("Third element: {}", third);
    } else {
        println!("Iterator exhausted.");
    }
} 