# Iterator Exhaustion Panic in Rust

This repository demonstrates a common error in Rust when working with iterators: attempting to access elements after the iterator has been exhausted.  The `bug.rs` file contains code that will panic, while `bugSolution.rs` provides a corrected version.

The error occurs because iterators in Rust are consumed as you use them. Once all elements have been accessed, they are exhausted and any further attempts to retrieve elements will result in a panic.  This example showcases how to correctly handle iterator exhaustion and avoid unexpected panics.