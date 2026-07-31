# TODO: number_guessing_game (⭐ 2/10)

## Usage

```bash
cargo run --bin number_guessing_game
cargo test --bin number_guessing_game
```

## Milestones

- [ ] Generate a random number between 1-100.
- [ ] Read user guesses from stdin.
- [ ] Compare guess to target and provide feedback (higher/lower/correct).
- [ ] Track number of attempts.
- [ ] Declare victory and show final count.

## Extra

- [ ] Add difficulty levels (smaller/larger ranges).
- [ ] Add max attempts limit.

## Tips

- Use `rand::random()` for number generation.
- Use `io::stdin().read_line()` for input parsing.
- Keep game loop simple and testable.
