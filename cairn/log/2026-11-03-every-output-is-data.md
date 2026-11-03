---
cairn: log
change: every-output-is-data
landed: 2026-11-03
---

# Every output is data

machine-output-contract gave the family one transport but said nothing about what a payload holds; the review of 2026-11-03 found every binary answering differently, with data hidden in `{"message": ...}` sentences that no schema could describe. `printer::Message` is gone, so the compiler lists every site a binary still owes.

The printer gains `event` (one flushed JSON line under `--json`, text on a terminal only) and `out_bytes` (verbatim in text, `{data, size, mime}` in base64 under `--json`). The new `output` module holds `Bytes` and `Paginated` (himalaya's, moved up, `nextPage` always printed), and `PageTokenArg` gives `--page-token`. The failure object prints `code: null` when the chain carries none. `PartialOutcome` lets a command that printed its result exit 2 or above without a failure object, `eval_coded` reading it from the chain.

`JsonSchemaRegistry` gains an `events` map (`<key>.events.json`), `new` (registering the toolkit commands' outputs, which are now untagged alternatives with schemas), `insert` and `insert_events`, built with the serialize contract so a key always printed is required, and `check`, which walks the clap command tree. During implementation the check had to exempt `<bin>-error` and count a command with an optional subcommand as runnable, and the delta was amended to say so, with the serialize contract and the partial-outcome marker.

Amended during the work, on the user's answers: raw-faithfulness applies to JSON payloads only; a text protocol's reply or a local store's value is structured when understood and text otherwise, in an alternative told apart by its keys, with no `raw` field beside a parsed reply (that would need io-imap, io-smtp and io-managesieve to surface response bytes); inside a protocol namespace, responses before the completion are events at the granularity the command observes, and a refused completion is a result exiting 2; `id` is `null` when queued or not returned.

Verified: 37 unit tests and the doc test pass, clippy clean with all features, `--no-default-features --features terminal` builds, docs build. himalaya migrated in the same session against a path patch (its change the-outputs-become-data), which exercised every API above.

Spec: output (16 requirements added), error (2 modified).

Left: each other binary's own change, listed in tasks.md; a release of 0.3.0 for the binaries to pin.
