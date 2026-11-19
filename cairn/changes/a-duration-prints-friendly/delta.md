---
cairn: delta
change: a-duration-prints-friendly
---

## ADDED Requirements

### Requirement: A duration prints friendly
`duration::friendly` SHALL print a count of whole seconds in jiff's friendly compact form (`{span:#}`), the span balanced up to days of 24 hours: `2d 4h 30m`, `0s` when empty. A count too large for a jiff span SHALL print as jiff's `SpanPrinter` prints the unsigned duration, its largest unit the hour. It SHALL be available with the `duration` feature, whose only dependency is jiff.

#### Scenario: A token lifetime and a time left
- GIVEN the durations 90, 3600, 5430 and 187 261 seconds
- WHEN each goes through `duration::friendly`
- THEN they read `1m 30s`, `1h`, `1h 30m 30s` and `2d 4h 1m 1s`

#### Scenario: A duration past a span
- GIVEN the duration `u64::MAX` seconds
- WHEN it goes through `duration::friendly`
- THEN it reads `5124095576030431h 15s`

## REMOVED Requirements
