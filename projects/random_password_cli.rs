// example must have main function
// Goal: Rust basics

// Build:

// ```bash
// genpass --length 20
// ```

// Learn:

// - Cargo
// - `String`
// - `Vec`
// - `rand`
// - argument parsing

// Extra:

// - support symbols toggle

use rand::Rng;

// cargo run --bin genpass -- --length 20
// genpass --length 20
fn main() {
    let args: Vec<String> = std::env::args().collect();
    // std::env::args() Before collect()
    // Args { inner: ["target/debug/genpass", "--length", "20"] }
    // Args { inner: ["genpass", "--length", "20"] }
    // after collect then Vec but the Vec need explicitly mention type
    println!("{:?}", args);
    // get the 2nd element and wrap out from Some
    if let Some(first_option) = args.get(1) {
        if first_option == "--length" {
            if let Some(second_option) = args.get(2) {
                // try parse to i32
                match second_option.parse::<i32>() {
                    Ok(length) => {
                        // Printable ASCII characters are from byte 33 to 126.
                        // letters
                        // numbers
                        // punctuation
                        // symbols
                        let chars: Vec<char> = (33u8..=126u8).map(char::from).collect();
                        // random_range needs to mutate rng internally so the next random 
                        // number is different from the previous one. 
                        // current random state -> generate number -> update state -> next state
                        let mut rng = rand::rng();
                        let mut password = String::new();

                        for _ in 0..length {
                            let index = rng.random_range(0..chars.len());
                            password.push(chars[index]);
                        }

                        println!("{password}");
                    }
                    Err(error) => println!("not a valid i32: {error}"),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(4, 4);
    }
}
