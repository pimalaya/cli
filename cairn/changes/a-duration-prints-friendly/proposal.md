---
cairn: change
id: a-duration-prints-friendly
status: landed
created: 2026-11-19
---

# A duration prints friendly

## Why

ortie (src/token/issued.rs, the token lifetime of its receipt) and comodoro (src/cli/client/timer.rs, the time a timer has left) each copied the same `friendly` formatter: whole seconds in jiff's friendly compact form, balanced up to days of 24 hours, falling back to hours past what a jiff span holds. Owner ruling of 2026-11-19: one kit function, so the apps share it.

## What

A `duration` feature, its only dependency jiff 1 with `std`, holding `duration::friendly(secs: u64) -> String`, the copied function as is. Tested on the union of both apps' tables. Additive, for 0.7.1.

comodoro's `truncate` (the remaining time cut to the account's `TimerPrecision`, down to the unit under it when the precision's own would read zero) stays in comodoro: it rests on comodoro's precision setting, and its fallback is a choice of that status line, not a rule the family shares.
