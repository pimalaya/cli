---
cairn: change
id: repl-line-editor
status: active
created: 2026-11-04
---

# Line editing for the REPLs

## Why

`ortie repl` and `himalaya repl` read their lines with a plain `read_line`: no cursor movement, no Emacs keys, no history. `rlwrap ortie repl` gives all of it today with no code, but a human at the prompt should not need a wrapper. The two REPL loops are already copies of each other, so the editor belongs here, where both can take it and where the loop itself is planned to move.

## What

1. A line reader behind a `repl` feature: reedline (nushell's line editor, MIT, on crossterm) when stdin and stdout are both terminals, a plain `read_line` otherwise. A piped session, the way an application keeps one alive, behaves exactly as today, and `ortie repl | jq` never gets the editor drawn into the pipe.
2. History in memory only, never written to disk: an `ortie auth resume <redirected URI>` line carries an authorization code.
3. ortie and himalaya take the reader in their REPL loops, the feature on by default as ortie's `wizard` is.

## Checked 2026-11-04

- reedline 0.52.1 uses crossterm 0.29, the version this crate's `spinner` feature already pulls: one crossterm in the build.
- reedline 0.52 needs Rust 1.95; this crate and its consumers declare 1.89. Raise the requirement for builds enabling the feature rather than let the resolver fall back to reedline 0.45 (Rust 1.63), which may bring a second crossterm.

## Open

- Whether reedline can draw on stderr, where the REPLs render their prompt today.
- Moving the whole REPL loop here (splitting, TTY detection, quit, failure printing) in the same change or after.

## Not in scope

A standalone readline wrapper in the manner of rlwrap (dropped 2026-11-04).
