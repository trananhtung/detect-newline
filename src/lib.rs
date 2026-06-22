//! # detect-newline — detect and normalize line endings
//!
//! Detect which line ending a string uses (`\n`, `\r\n`, or `\r`), normalize a
//! string to one style, count the styles, iterate lines (splitting on all three),
//! and trim trailing newlines. Zero dependencies and `no_std`. A Rust take on
//! Node's [`detect-newline`](https://www.npmjs.com/package/detect-newline).
//!
//! ```
//! use detect_newline::{detect, dominant, to_lf, Newline};
//!
//! assert_eq!(detect("a\r\nb"), Some(Newline::CrLf));   // first newline found
//! assert_eq!(dominant("a\nb\nc\r\nd"), Some(Newline::Lf)); // most common
//! assert_eq!(to_lf("a\r\nb\rc"), "a\nb\nc");            // normalize
//! ```
//!
//! `\r\n` is always treated as a single line ending, never as a `\r` followed by a
//! `\n`. [`detect`] returns the first ending found; [`dominant`] returns the most
//! frequent (ties broken by which occurs first).

#![no_std]
#![doc(html_root_url = "https://docs.rs/detect-newline/0.1.0")]

extern crate alloc;

use alloc::string::String;
use core::iter::FusedIterator;

// Compile-test the README's examples as part of `cargo test`.
#[cfg(doctest)]
#[doc = include_str!("../README.md")]
struct ReadmeDoctests;

/// A line-ending style.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Newline {
    /// Unix line feed, `\n`. The default.
    #[default]
    Lf,
    /// Windows carriage return + line feed, `\r\n`.
    CrLf,
    /// Classic Mac OS carriage return, `\r`.
    Cr,
}

impl Newline {
    /// The characters this line ending is written as.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Newline::Lf => "\n",
            Newline::CrLf => "\r\n",
            Newline::Cr => "\r",
        }
    }
}

/// How many of each line-ending style a string contains. See [`count`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Counts {
    /// Number of `\n` line endings.
    pub lf: usize,
    /// Number of `\r\n` line endings.
    pub crlf: usize,
    /// Number of lone `\r` line endings.
    pub cr: usize,
}

/// Detect the first line ending in `text`, or `None` if it has none.
///
/// ```
/// assert_eq!(detect_newline::detect("a\r\nb"), Some(detect_newline::Newline::CrLf));
/// ```
#[must_use]
pub fn detect(text: &str) -> Option<Newline> {
    let bytes = text.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        match bytes[i] {
            b'\r' => {
                return Some(if bytes.get(i + 1) == Some(&b'\n') {
                    Newline::CrLf
                } else {
                    Newline::Cr
                });
            }
            b'\n' => return Some(Newline::Lf),
            _ => i += 1,
        }
    }
    None
}

/// Count each line-ending style in `text`.
#[must_use]
pub fn count(text: &str) -> Counts {
    let bytes = text.as_bytes();
    let mut c = Counts::default();
    let mut i = 0;
    while i < bytes.len() {
        match bytes[i] {
            b'\r' => {
                if bytes.get(i + 1) == Some(&b'\n') {
                    c.crlf += 1;
                    i += 2;
                } else {
                    c.cr += 1;
                    i += 1;
                }
            }
            b'\n' => {
                c.lf += 1;
                i += 1;
            }
            _ => i += 1,
        }
    }
    c
}

/// Detect the most frequent line ending in `text`, or `None` if it has none.
///
/// Ties are broken by which style occurs first.
#[must_use]
pub fn dominant(text: &str) -> Option<Newline> {
    let bytes = text.as_bytes();
    // (count, first-occurrence index) per style.
    let mut stats = [
        (0usize, usize::MAX, Newline::Lf),
        (0usize, usize::MAX, Newline::CrLf),
        (0usize, usize::MAX, Newline::Cr),
    ];
    let mut i = 0;
    while i < bytes.len() {
        match bytes[i] {
            b'\r' => {
                let (slot, step) = if bytes.get(i + 1) == Some(&b'\n') {
                    (1, 2)
                } else {
                    (2, 1)
                };
                bump(&mut stats[slot], i);
                i += step;
            }
            b'\n' => {
                bump(&mut stats[0], i);
                i += 1;
            }
            _ => i += 1,
        }
    }

    let mut best: Option<(usize, usize, Newline)> = None;
    for &(c, idx, nl) in &stats {
        if c == 0 {
            continue;
        }
        let better = match best {
            None => true,
            Some((bc, bi, _)) => c > bc || (c == bc && idx < bi),
        };
        if better {
            best = Some((c, idx, nl));
        }
    }
    best.map(|(_, _, nl)| nl)
}

/// Record one occurrence of a style at byte index `at`.
fn bump(slot: &mut (usize, usize, Newline), at: usize) {
    if slot.0 == 0 {
        slot.1 = at;
    }
    slot.0 += 1;
}

/// Replace every line ending in `text` with `to`.
///
/// ```
/// use detect_newline::{normalize, Newline};
/// assert_eq!(normalize("a\r\nb\rc", Newline::Lf), "a\nb\nc");
/// ```
#[must_use]
pub fn normalize(text: &str, to: Newline) -> String {
    let nl = to.as_str();
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    loop {
        match rest.find(['\n', '\r']) {
            None => {
                out.push_str(rest);
                break;
            }
            Some(i) => {
                out.push_str(&rest[..i]);
                out.push_str(nl);
                let bytes = rest.as_bytes();
                let adv = if bytes[i] == b'\r' && bytes.get(i + 1) == Some(&b'\n') {
                    i + 2
                } else {
                    i + 1
                };
                rest = &rest[adv..];
            }
        }
    }
    out
}

/// Normalize every line ending in `text` to `\n`.
#[must_use]
pub fn to_lf(text: &str) -> String {
    normalize(text, Newline::Lf)
}

/// Normalize every line ending in `text` to `\r\n`.
#[must_use]
pub fn to_crlf(text: &str) -> String {
    normalize(text, Newline::CrLf)
}

/// Normalize every line ending in `text` to `\r`.
#[must_use]
pub fn to_cr(text: &str) -> String {
    normalize(text, Newline::Cr)
}

/// Whether `text` ends with a line ending.
#[must_use]
pub fn has_trailing_newline(text: &str) -> bool {
    text.ends_with(['\n', '\r'])
}

/// Strip a single trailing line ending (`\r\n`, `\n`, or `\r`) from `text`.
#[must_use]
pub fn strip_trailing_newline(text: &str) -> &str {
    if let Some(s) = text.strip_suffix("\r\n") {
        s
    } else if let Some(s) = text.strip_suffix(['\n', '\r']) {
        s
    } else {
        text
    }
}

/// Iterate the lines of `text`, splitting on `\n`, `\r\n`, and lone `\r`.
///
/// Like [`str::lines`] but also splitting on a lone `\r`; a single trailing line
/// ending does not yield a final empty line.
///
/// ```
/// use detect_newline::lines;
/// assert_eq!(lines("a\r\nb\rc").collect::<Vec<_>>(), ["a", "b", "c"]);
/// ```
#[must_use]
pub fn lines(text: &str) -> Lines<'_> {
    Lines {
        rest: text,
        done: false,
    }
}

/// Iterator over the lines of a string. Created by [`lines`].
#[derive(Debug, Clone)]
pub struct Lines<'a> {
    rest: &'a str,
    done: bool,
}

impl<'a> Iterator for Lines<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<&'a str> {
        if self.done {
            return None;
        }
        match self.rest.find(['\n', '\r']) {
            None => {
                self.done = true;
                if self.rest.is_empty() {
                    None
                } else {
                    let line = self.rest;
                    self.rest = "";
                    Some(line)
                }
            }
            Some(i) => {
                let line = &self.rest[..i];
                let bytes = self.rest.as_bytes();
                let adv = if bytes[i] == b'\r' && bytes.get(i + 1) == Some(&b'\n') {
                    i + 2
                } else {
                    i + 1
                };
                let after = &self.rest[adv..];
                if after.is_empty() {
                    self.done = true; // trailing line ending → no empty final line
                }
                self.rest = after;
                Some(line)
            }
        }
    }
}

impl FusedIterator for Lines<'_> {}
