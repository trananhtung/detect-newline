//! End-to-end behavioral spec for the public `detect-newline` API.

use detect_newline::{
    count, detect, dominant, has_trailing_newline, lines, normalize, strip_trailing_newline,
    to_crlf, to_lf, Newline,
};

// ---------------------------------------------------------------------------
// Newline
// ---------------------------------------------------------------------------

#[test]
fn newline_as_str() {
    assert_eq!(Newline::Lf.as_str(), "\n");
    assert_eq!(Newline::CrLf.as_str(), "\r\n");
    assert_eq!(Newline::Cr.as_str(), "\r");
    assert_eq!(Newline::default(), Newline::Lf);
}

// ---------------------------------------------------------------------------
// detect() — first newline wins
// ---------------------------------------------------------------------------

#[test]
fn detect_first_newline() {
    assert_eq!(detect("a\nb"), Some(Newline::Lf));
    assert_eq!(detect("a\r\nb"), Some(Newline::CrLf));
    assert_eq!(detect("a\rb"), Some(Newline::Cr));
    assert_eq!(detect("abc"), None);
    assert_eq!(detect(""), None);
    // CRLF must be recognized as one unit, not CR then LF
    assert_eq!(detect("a\r\nb\nc"), Some(Newline::CrLf));
    assert_eq!(detect("a\nb\r\nc"), Some(Newline::Lf));
    // lone CR at end of buffer
    assert_eq!(detect("ab\r"), Some(Newline::Cr));
}

// ---------------------------------------------------------------------------
// count()
// ---------------------------------------------------------------------------

#[test]
fn count_each_style() {
    let c = count("a\nb\r\nc\rd");
    assert_eq!((c.lf, c.crlf, c.cr), (1, 1, 1));
    // CRLF must not be miscounted as a CR plus an LF
    let c = count("\r\n\r\n");
    assert_eq!((c.lf, c.crlf, c.cr), (0, 2, 0));
    let c = count("plain");
    assert_eq!((c.lf, c.crlf, c.cr), (0, 0, 0));
}

// ---------------------------------------------------------------------------
// dominant() — most frequent, ties broken by first occurrence
// ---------------------------------------------------------------------------

#[test]
fn dominant_most_frequent() {
    assert_eq!(dominant("a\nb\nc\r\nd"), Some(Newline::Lf)); // 2 LF vs 1 CRLF
    assert_eq!(dominant("a\r\nb\r\nc\nd"), Some(Newline::CrLf)); // 2 CRLF vs 1 LF
    assert_eq!(dominant("abc"), None);
    // tie → the style that occurs first (LF at index 1 before the CRLF)
    assert_eq!(dominant("a\nb\r\n"), Some(Newline::Lf));
    assert_eq!(dominant("a\r\nb\n"), Some(Newline::CrLf));
}

// ---------------------------------------------------------------------------
// normalize() / to_lf / to_crlf
// ---------------------------------------------------------------------------

#[test]
fn normalize_converts_all_styles() {
    assert_eq!(normalize("a\r\nb\nc\rd", Newline::Lf), "a\nb\nc\nd");
    assert_eq!(normalize("a\nb", Newline::CrLf), "a\r\nb");
    assert_eq!(to_lf("a\r\nb\rc"), "a\nb\nc");
    assert_eq!(to_crlf("a\nb\r\nc"), "a\r\nb\r\nc");
    // no newlines → unchanged; preserves non-ASCII
    assert_eq!(normalize("héllo 世界", Newline::Lf), "héllo 世界");
}

#[test]
fn normalize_is_idempotent() {
    let once = to_lf("a\r\nb\rc\nd");
    assert_eq!(to_lf(&once), once);
}

// ---------------------------------------------------------------------------
// lines() — splits on LF, CRLF, and lone CR
// ---------------------------------------------------------------------------

#[test]
fn lines_splits_all_styles() {
    assert_eq!(
        lines("a\nb\r\nc\rd").collect::<Vec<_>>(),
        vec!["a", "b", "c", "d"]
    );
    assert_eq!(lines("a\n").collect::<Vec<_>>(), vec!["a"]); // trailing newline → no empty line
    assert_eq!(lines("a\n\nb").collect::<Vec<_>>(), vec!["a", "", "b"]);
    assert_eq!(lines("abc").collect::<Vec<_>>(), vec!["abc"]);
    assert_eq!(lines("").collect::<Vec<_>>(), Vec::<&str>::new());
}

// ---------------------------------------------------------------------------
// trailing-newline helpers
// ---------------------------------------------------------------------------

#[test]
fn trailing_newline_helpers() {
    assert!(has_trailing_newline("a\n"));
    assert!(has_trailing_newline("a\r\n"));
    assert!(has_trailing_newline("a\r"));
    assert!(!has_trailing_newline("a"));
    assert!(!has_trailing_newline(""));

    assert_eq!(strip_trailing_newline("a\r\n"), "a");
    assert_eq!(strip_trailing_newline("a\n"), "a");
    assert_eq!(strip_trailing_newline("a\r"), "a");
    assert_eq!(strip_trailing_newline("a"), "a");
    assert_eq!(strip_trailing_newline("a\n\n"), "a\n"); // only one stripped
}
