# Changelog

All notable changes to this project are documented here. The format is based on
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project
adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-06-22

### Added

- Initial release.
- `detect` / `dominant` / `count` — find the first, most frequent, or all
  line-ending styles in a string.
- `normalize` / `to_lf` / `to_crlf` / `to_cr` — convert every line ending to one
  style (UTF-8 safe, idempotent).
- `lines` — iterate lines, splitting on `\n`, `\r\n`, and a lone `\r`.
- `has_trailing_newline` / `strip_trailing_newline` — trailing-ending helpers.
- `Newline` enum (`Lf` default, `CrLf`, `Cr`) and `Counts` struct.
- Zero dependencies; `#![no_std]` (requires `alloc`).

[0.1.0]: https://github.com/trananhtung/detect-newline/releases/tag/v0.1.0
