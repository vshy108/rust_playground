// Calculator solution flow:
// 1. Read one expression from standard input, such as `2 + 3 * (4 - 1)`.
// 2. Tokenize the text into numbers, operators, and parentheses.
// 3. Parse the tokens with recursive-descent rules so `*` and `/` take precedence over `+` and `-`.
// 4. Apply checked integer arithmetic while parsing, returning errors for invalid input, overflow,
//    missing parentheses, and division by zero.
// 5. Print either the final integer result or the error message.
use std::io;

// Tokens are the small, meaningful pieces of an expression that the parser understands.
#[derive(Debug, PartialEq, Eq)]
enum Token {
    // The contained i32 is the actual value read from the user's input.
    #[allow(dead_code)]
    Number(i32),
    Add,
    Subtract,
    Multiply,
    Divide,
    LeftParen,
    RightParen,
}

fn main() {
    // Keep I/O and error printing at the edge of the program.
    print_result(run());
}

fn run() -> Result<i32, String> {
    // The work happens in two stages: convert text into tokens, then evaluate those tokens.
    let input = read_input()?;
    let tokens = tokenize(input.trim())?;
    evaluate(&tokens)
}

fn print_result(result: Result<i32, String>) {
    match result {
        Ok(value) => println!("Result: {value}"),
        Err(error) => eprintln!("Error: {error}"),
    }
}

fn read_input() -> Result<String, String> {
    println!("Enter an expression to evaluate:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .map_err(|error| format!("failed to read input: {error}"))?;

    Ok(input)
}

fn tokenize(input: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();

    // `char_indices` supplies byte positions as well as characters, which makes error messages
    // point to the problematic input and lets us safely slice multi-digit numbers below.
    let mut characters = input.char_indices().peekable();

    while let Some((index, character)) = characters.next() {
        let token = match character {
            character if character.is_whitespace() => continue,
            '+' => Token::Add,
            '-' => Token::Subtract,
            '*' => Token::Multiply,
            '/' => Token::Divide,
            '(' => Token::LeftParen,
            ')' => Token::RightParen,
            character if character.is_ascii_digit() => {
                let start = index;
                let mut end = index + character.len_utf8();

                // Consume the complete run of digits so `123` becomes one Number token.
                while let Some(&(next_index, next_character)) = characters.peek() {
                    if !next_character.is_ascii_digit() {
                        break;
                    }

                    characters.next();
                    end = next_index + next_character.len_utf8();
                }

                let number = input[start..end]
                    .parse::<i32>()
                    .map_err(|_| format!("number at position {start} is outside the i32 range"))?;
                Token::Number(number)
            }
            _ => {
                return Err(format!(
                    "unexpected character '{character}' at position {index}"
                ))
            }
        };

        tokens.push(token);
    }

    // An empty token list cannot form a valid expression.
    if tokens.is_empty() {
        return Err("expression is empty".to_string());
    }

    Ok(tokens)
}

fn evaluate(tokens: &[Token]) -> Result<i32, String> {
    // Start with the lowest-precedence rule. Each rule delegates to a higher-precedence rule.
    let mut parser = Parser {
        tokens,
        position: 0,
    };
    let value = parser.parse_expression()?;

    if parser.peek().is_some() {
        return Err("unexpected token after complete expression".to_string());
    }

    Ok(value)
}

// Grammar implemented by this recursive-descent parser:
// expression = term (("+" | "-") term)*
// term       = unary (("*" | "/") unary)*
// unary      = ("+" | "-") unary | primary
// primary    = number | "(" expression ")"
struct Parser<'a> {
    tokens: &'a [Token],
    position: usize,
}

impl Parser<'_> {
    fn parse_expression(&mut self) -> Result<i32, String> {
        // Addition and subtraction are evaluated after multiplication and division.
        let mut value = self.parse_term()?;

        loop {
            match self.peek() {
                Some(Token::Add) => {
                    self.position += 1;
                    value = checked_add(value, self.parse_term()?)?;
                }
                Some(Token::Subtract) => {
                    self.position += 1;
                    value = checked_subtract(value, self.parse_term()?)?;
                }
                _ => return Ok(value),
            }
        }
    }

    fn parse_term(&mut self) -> Result<i32, String> {
        // Parse a complete unary value before applying multiplication or division.
        let mut value = self.parse_unary()?;

        loop {
            match self.peek() {
                Some(Token::Multiply) => {
                    self.position += 1;
                    value = checked_multiply(value, self.parse_unary()?)?;
                }
                Some(Token::Divide) => {
                    self.position += 1;
                    let divisor = self.parse_unary()?;
                    value = checked_divide(value, divisor)?;
                }
                _ => return Ok(value),
            }
        }
    }

    fn parse_unary(&mut self) -> Result<i32, String> {
        // Recursing here supports expressions such as `--5` and `-(2 + 3)`.
        match self.peek() {
            Some(Token::Add) => {
                self.position += 1;
                self.parse_unary()
            }
            Some(Token::Subtract) => {
                self.position += 1;
                self.parse_unary()?
                    .checked_neg()
                    .ok_or_else(|| "integer overflow while negating value".to_string())
            }
            _ => self.parse_primary(),
        }
    }

    fn parse_primary(&mut self) -> Result<i32, String> {
        // A primary is the simplest expression: a number or a parenthesized expression.
        match self.next() {
            Some(Token::Number(number)) => Ok(*number),
            Some(Token::LeftParen) => {
                let value = self.parse_expression()?;

                match self.next() {
                    Some(Token::RightParen) => Ok(value),
                    _ => Err("missing closing parenthesis".to_string()),
                }
            }
            Some(Token::RightParen) => Err("unexpected closing parenthesis".to_string()),
            Some(_) => Err("expected a number or opening parenthesis".to_string()),
            None => Err("expected an expression".to_string()),
        }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }

    fn next(&mut self) -> Option<&Token> {
        let token = self.tokens.get(self.position);

        // Only advance when a token exists so repeated calls at end-of-input remain safe.
        self.position += usize::from(token.is_some());
        token
    }
}

// Rust's checked arithmetic returns None on i32 overflow; convert that into a clear CLI error.
fn checked_add(left: i32, right: i32) -> Result<i32, String> {
    left.checked_add(right)
        .ok_or_else(|| "integer overflow during addition".to_string())
}

fn checked_subtract(left: i32, right: i32) -> Result<i32, String> {
    left.checked_sub(right)
        .ok_or_else(|| "integer overflow during subtraction".to_string())
}

fn checked_multiply(left: i32, right: i32) -> Result<i32, String> {
    left.checked_mul(right)
        .ok_or_else(|| "integer overflow during multiplication".to_string())
}

fn checked_divide(dividend: i32, divisor: i32) -> Result<i32, String> {
    if divisor == 0 {
        return Err("division by zero".to_string());
    }

    dividend
        .checked_div(divisor)
        .ok_or_else(|| "integer overflow during division".to_string())
}

#[cfg(test)]
mod tests {
    use super::{evaluate, tokenize, Token};

    fn calculate(input: &str) -> Result<i32, String> {
        let tokens = tokenize(input)?;
        evaluate(&tokens)
    }

    #[test]
    fn tokenizes_compact_expressions() {
        assert_eq!(
            tokenize("12+(3*4)").unwrap(),
            vec![
                Token::Number(12),
                Token::Add,
                Token::LeftParen,
                Token::Number(3),
                Token::Multiply,
                Token::Number(4),
                Token::RightParen,
            ]
        );
    }

    #[test]
    fn evaluates_operator_precedence() {
        assert_eq!(calculate("2 + 3 * 4 - 8 / 2"), Ok(10));
    }

    #[test]
    fn evaluates_parentheses_and_unary_operators() {
        assert_eq!(calculate("-(2 + 3) * -4"), Ok(20));
    }

    #[test]
    fn reports_invalid_expressions() {
        assert_eq!(calculate("2 +"), Err("expected an expression".to_string()));
        assert_eq!(
            calculate("(2 + 3"),
            Err("missing closing parenthesis".to_string())
        );
        assert_eq!(calculate("2 / 0"), Err("division by zero".to_string()));
    }

    #[test]
    fn reports_unexpected_characters_and_overflow() {
        assert_eq!(
            tokenize("2 ^ 3"),
            Err("unexpected character '^' at position 2".to_string())
        );
        assert_eq!(
            calculate("2147483647 + 1"),
            Err("integer overflow during addition".to_string())
        );
    }
}
