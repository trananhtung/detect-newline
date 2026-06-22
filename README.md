# detect-newline

[![Crates.io](https://img.shields.io/crates/v/detect-newline.svg)](https://crates.io/crates/detect-newline)
[![Documentation](https://docs.rs/detect-newline/badge.svg)](https://docs.rs/detect-newline)
[![CI](https://github.com/trananhtung/detect-newline/actions/workflows/ci.yml/badge.svg)](https://github.com/trananhtung/detect-newline/actions/workflows/ci.yml)
[![License](https://img.shields.io/crates/l/detect-newline.svg)](#license)

**Detect and normalize line endings** — `\n` (LF), `\r\n` (CRLF), and `\r` (CR).
Zero dependencies and `no_std`. A Rust take on Node's
[`detect-newline`](https://www.npmjs.com/package/detect-newline), plus the
conversion and line-splitting you usually reach for next.

```rust
use detect_newline::{detect, dominant, to_lf, lines, Newline};

assert_eq!(detect("a\r\nb"), Some(Newline::CrLf));        // first ending found
assert_eq!(dominant("a\nb\nc\r\nd"), Some(Newline::Lf));  // most common
assert_eq!(to_lf("a\r\nb\rc"), "a\nb\nc");                // normalize
assert_eq!(lines("a\r\nb\rc").collect::<Vec<_>>(), ["a", "b", "c"]); // CR-aware
```

## Why detect-newline?

The Rust standard library's `str::lines` splits on `\n`/`\r\n` but not a lone `\r`,
and there's no built-in way to *detect* a string's line-ending style or normalize a
mixed string to one. This crate is the small, focused piece — pick a window of text
apart by line ending, decide which one a file uses, and convert between them.

```toml
[dependencies]
detect-newline = "0.1"
```

## API

| Item | Purpose |
| --- | --- |
| `detect(text)` | The first line ending, or `None` |
| `dominant(text)` | The most frequent line ending (ties → first occurrence) |
| `count(text)` | `Counts { lf, crlf, cr }` of each style |
| `normalize(text, to)` / `to_lf` / `to_crlf` / `to_cr` | Convert every ending to one style |
| `lines(text)` | Iterate lines, splitting on `\n`, `\r\n`, and lone `\r` |
| `has_trailing_newline(text)` / `strip_trailing_newline(text)` | Trailing-ending helpers |
| `Newline` | `Lf` (default), `CrLf`, `Cr` |

## Behavior

- `\r\n` is always one line ending, never a `\r` followed by a `\n` — in detection,
  counting, normalization, and splitting alike.
- `detect` returns the first ending found; `dominant` returns the most frequent,
  breaking ties toward whichever style occurs first.
- `normalize` operates on character boundaries (safe for any UTF-8) and is
  idempotent.
- `lines` mirrors `str::lines` but also splits a lone `\r`; a single trailing line
  ending does not yield a final empty line.

## `no_std`

`#![no_std]` (needs only `alloc` for `String`) with zero dependencies.

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at
your option.
