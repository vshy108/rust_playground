fn main() {
    println!("Hello, world!");
    // rust_playground can refer to src/lib.rs
    println!("Sample main.rs read: {}", rust_playground::add(1, 2));
    // `cargo run` to check error
    // cargo clippy --all-targets --all-features -- -D warnings
}
