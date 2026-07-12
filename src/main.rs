fn main() {
    println!("Hello, world!");
    // rust_playground can refer to src/lib.rs
    println!("Sample main.rs read: {}", rust_playground::add(1, 2));
    // `cargo run` to check error
    // cargo clippy --all-targets --all-features -- -D warnings

    // prints the shortest round-trip representation hence 5f64.sqrt() less decimal places
    // 3.141592653589793
    println!("{}", std::f64::consts::PI);
    // 2.23606797749979
    println!("{}", 5f64.sqrt());
}
