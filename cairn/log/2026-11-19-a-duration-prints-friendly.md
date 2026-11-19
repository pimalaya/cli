---
cairn: log
change: a-duration-prints-friendly
landed: 2026-11-19
---

# A duration prints friendly

Owner ruling of 2026-11-19: the duration formatter ortie and comodoro each copied moves to the kit.

## What landed

- The `duration` feature, jiff 1 (`std`) its only dependency.
- `duration::friendly` (src/duration.rs), the copied function as is: jiff's friendly compact form balanced up to days of 24 hours, `SpanPrinter`'s hours past a jiff span.
- `a_duration_prints_in_days_at_most` and `a_duration_past_a_span_prints_in_hours`, the union of both apps' tables (comodoro's cases at their truncated values).

Additive, for 0.7.1. comodoro's `truncate` stays in comodoro: it rests on its `TimerPrecision` setting.

## Capabilities moved

- [terminal](../spec/terminal.md): *A duration prints friendly* added.
