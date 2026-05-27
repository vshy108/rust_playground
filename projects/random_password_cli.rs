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

// function order no matter before or after main, but before is better
fn generate_password(length: usize) -> String {
    // logic
    // Printable ASCII characters are from byte 33 to 126.
    // !"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\]^_`abcdefghijklmnopqrstuvwxyz{|}~
    // letters
    // numbers
    // punctuation
    // symbols
    let chars: Vec<char> = (33u8..=126u8).map(char::from).collect();
    // random_range needs to mutate rng internally so the next random
    // number is different from the previous one.
    // current random state -> generate number -> update state -> next state
    let mut rng = rand::rng();
    // know the final size, can pre-allocate
    let mut password = String::with_capacity(length);
    // Vec::len() is very cheap.
    // It just reads a stored number from the Vec; it does not count the elements one by one
    // avoid even that tiny repeated call
    let chars_len = chars.len();

    for _ in 0..length {
        let index = rng.random_range(0..chars_len);
        password.push(chars[index]);
    }

    return password;
}

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
                // try parse to ~i32~ usize because fuction argument type is usize
                match second_option.parse::<usize>() {
                    Ok(length) => {
                        // NOTE: do not allow calling a function inside the {...}
                        println!("{}", generate_password(length))
                    }
                    Err(error) => println!("not a valid i32: {error}"),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    // super::generate_password
    use super::*;

    #[test]
    fn generates_password_with_requested_length() {
        let password = generate_password(20);

        assert_eq!(password.len(), 20);
    }

    #[test]
    fn generated_password_uses_printable_ascii_without_space() {
        let password = generate_password(100);

        // check all fulfill condition
        assert!(password.chars().all(|ch| ch >= '!' && ch <= '~'));
    }
}
