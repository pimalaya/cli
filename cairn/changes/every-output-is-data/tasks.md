---
cairn: tasks
change: every-output-is-data
---

# Tasks

## pimalaya-cli

- [x] Remove `Message` from the printer module
- [x] `Printer::event`: one flushed line under `--json`; `Display` on a terminal in text mode, nothing when piped; `TestPrinter` records events apart from the result
- [x] Bytes type `{data, size, mime}`, `data` base64, its schema declaring `contentEncoding: base64`
- [x] `--page-token` argument and a `nextPage` wrapper flattening a listing beside its cursor (himalaya's `Paginated`, moved here)
- [x] Registry: an `events` map beside `schemas`; `json-schema` writes `<key>.events.json` with its `$id`
- [x] Registry check over the clap tree: a leaf without a key and not declared silent, a key naming no leaf (both maps), a result root holding an `event` key, an event alternative without one
- [x] Register the `completion`, `manual` and `json-schema` outputs through the binary's registry
- [x] Failure object: `code` always printed, `null` when absent; schema marks it required and nullable
- [x] Exit code constants: 1 failure object, 2 and above a result whose outcome is not a full success
- [x] Tests for each of the above; CHANGELOG under Changed, `Message` under Removed, both breaking
- [x] Fold delta.md into cairn/spec/output.md and cairn/spec/error.md; log entry; status `landed`

## Each binary (its own change, at its next breaking release)

- [x] himalaya: the 111 `Message` sites (56 carrying data), raw bytes under `--json`, multi-shape commands, flags, `inReplyTo`, attachments, paging, schema required-ness, `pimdir tag` values, queue `row`, `account check` report and exit 2, IMAP/SMTP/Sieve replies structured when understood with refusals as results, `imap raw` responses as events (review of 2026-11-03; himalaya change the-outputs-become-data)
- [ ] cardamum: ETag and copied ids in prose, two-shape JMAP updates, provider tokens no flag accepts, legacy card fields, `displayName`, `account check`
- [ ] calendula: `item list` contents printed as numbers, agenda root keyed by data, `calendar create` id in prose, `""` for absent, `queued` meaning a row, `account check`
- [ ] journal: create id in prose, `entry build -o` off its schema, `""` for absent, `displayName`, `account check` exit
- [ ] unfog: `queued` and `parked` each with two types, `place` prose, `task` vs `uid`, untagged error codes, `account check` exit
- [x] neverest: `mode` prose, `null` vs omitted, sync progress as events, `check` printing every endpoint and exiting 2 (neverest changes sync-progress-is-events, every-key-is-printed, a-check-reports-every-endpoint)
- [ ] pimdir CLI: codes dropped, second failure shape, camelCase values, `queue status` unregistered, `next`/`nextOffset` paging
- [ ] ortie: data in prose, `auth get` shapes, epoch times, REPL help under `--json`
- [ ] comodoro: PascalCase values and `{"Fixed": n}`, epoch times, `watch` as events
- [ ] carillon: prose on stdout at Ctrl+C, `check` exit
- [ ] sirup: kebab-case `ConfigureOutput`, REPL ignoring `--json`
