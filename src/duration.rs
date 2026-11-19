//! Durations printed for a human.
//!
//! [`friendly`] prints a count of whole seconds the way the family words
//! a lifetime or a time left: jiff's friendly compact form, balanced up
//! to days of 24 hours.

use std::time::Duration;

use jiff::{Span, SpanRound, Unit, fmt::friendly::SpanPrinter};

/// Prints a duration in seconds in jiff's friendly compact form, balanced
/// up to days of 24 hours: `2d 4h 30m`, `0s` when empty.
///
/// A duration too long for a jiff span stops at hours.
pub fn friendly(secs: u64) -> String {
    let rounding = SpanRound::new().largest(Unit::Day).days_are_24_hours();

    let span = i64::try_from(secs)
        .ok()
        .and_then(|secs| Span::new().try_seconds(secs).ok())
        .and_then(|span| span.round(rounding).ok());

    match span {
        Some(span) => format!("{span:#}"),
        None => SpanPrinter::new().unsigned_duration_to_string(&Duration::from_secs(secs)),
    }
}

#[cfg(test)]
mod tests {
    use crate::duration::friendly;

    #[test]
    fn a_duration_prints_in_days_at_most() {
        let cases = [
            (0, "0s"),
            (59, "59s"),
            (60, "1m"),
            (90, "1m 30s"),
            (1500, "25m"),
            (3540, "59m"),
            (3600, "1h"),
            (5400, "1h 30m"),
            (5430, "1h 30m 30s"),
            (2 * 86_400 + 4 * 3600, "2d 4h"),
            (2 * 86_400 + 4 * 3600 + 61, "2d 4h 1m 1s"),
            (400 * 86_400 + 61, "400d 1m 1s"),
        ];

        for (secs, text) in cases {
            assert_eq!(friendly(secs), text, "{secs}");
        }
    }

    #[test]
    fn a_duration_past_a_span_prints_in_hours() {
        assert_eq!(friendly(u64::MAX), "5124095576030431h 15s");
    }
}
