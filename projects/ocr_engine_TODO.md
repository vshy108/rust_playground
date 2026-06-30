# TODO: ocr_engine (⭐ 8/10)

## Usage

```bash
cargo run --bin ocr_engine
cargo test --bin ocr_engine
```

## Milestones

- [ ] Implement image preprocessing pipeline for thresholding or segmentation.
- [ ] Model glyphs, lines, and recognition outputs.
- [ ] Add template-based or classifier-based character recognition.
- [ ] Implement confidence scoring and error reporting.
- [ ] Add support for reading text from sample fixture images.
- [ ] Add tests for preprocessing, glyph extraction, and expected text output.

## Extra

- [ ] Add language model post-processing or dictionary correction.

## Tips

- Preprocessing quality often matters more than the recognizer at first.
- Keep image ops and recognition stages independently testable.
- Fixture images with expected output are essential for regression checks.
- Confidence scores are useful even if recognition is initially simplistic.
