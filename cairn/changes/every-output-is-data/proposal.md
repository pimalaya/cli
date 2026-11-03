---
cairn: change
id: every-output-is-data
status: active
created: 2026-11-03
---

# Every output is data

## Why

machine-output-contract (landed 2026-10-27) gave the family one transport: newline-delimited JSON, a versioned schema registry, a coded failure object. It said nothing about what a payload holds, and a review of every binary on 2026-11-03 (himalaya at 2f49f86, cardamum, calendula, journal, unfog, neverest, the pimdir CLI, ortie, comodoro, carillon, sirup) found each one answering that question differently:

- **Prose carries data.** himalaya prints 111 `{"message": ...}` confirmations, 56 of them built from data: created ids, counts, server replies, warnings, message bodies. cardamum, calendula, journal and ortie do the same with ETags, created ids, paths and expiry times. A caller cannot read any of it back. neverest and the pimdir CLI never print a message and lose nothing by it.
- **No registry can be complete.** `Message` has no JSON Schema, so no binary can describe the 175 or so commands answering one, and no test can check that every command is described. himalaya has 110 leaves without a schema, about ten of which print data.
- **One command, several shapes.** Composers (template or message), `--raw` reads, a JMAP update answering an object or a message, ortie `auth get` answering a message for one grant.
- **One noun, several shapes.** A collection is `name` or `displayName`. `queued` is a count, a queue row id and a boolean. A queue row is `row`, `queueId` or `id`. Flags take five forms inside himalaya alone. `startsAt` is UTC in journal and local with an offset in calendula.
- **Absent is written three ways**: `null`, `""`, and an omitted key, sometimes inside one type.
- **Paging is four styles**: a reusable token, a token no flag accepts (Graph's `@odata.nextLink`, People's `nextPageToken` in cardamum), a `more` boolean, and nothing at all, where a caller stops on an empty page.
- **Times are four encodings**: RFC 3339 with `Z`, RFC 3339 with an offset, iCalendar text, epoch seconds (ortie, comodoro).
- **Bytes are four encodings**: base64 fields, lossy text, text failing on non-UTF-8, and raw bytes written to stdout under `--json` (himalaya Gmail and Graph raw reads), which breaks the one-value-per-line rule. calendula prints `item list` contents as an array of numbers.
- **Enum values are kebab-case, lowercase, camelCase (`unknownFlags`) or PascalCase (`"Running"`, `{"Fixed": n}`).**
- **A failed `check` either exits 1 with one joined error, losing each backend's code, or prints `ok: false` and exits 0.**

Separately, long jobs (a sync, a large fetch, a batch write) print nothing until they end. A machine caller has the exit code to say a job finished, and stdout only ever carries data or the failure object, so stdout can carry what happens during a run too, provided a caller can tell it from the result and learn its shape.

## What

The rules below bind every binary printing through pimalaya-cli. The spec keeps them in cairn/spec/output.md and cairn/spec/error.md; each binary's spec lists only its exceptions. pimalaya-cli enforces what a type or a test can enforce, so the rules are compiled and tested rather than reviewed.

### Every output is a described value

`Message` leaves pimalaya-cli. Every value a command prints is a named type deriving `Serialize` and `JsonSchema`, its `Display` writing the text form. Removing the type breaks every binary at compile time, which is the point: the compiler lists the sites. No field carries prose the binary composed: a sentence is the `Display` of data, written for text mode only. Text a server or a file provides (a server reply, a script warning, a provider error) is data and travels as a string field.

A write answers a receipt naming what it acted on, from one vocabulary:

| Key | Meaning |
|---|---|
| `id` | the item as it is after the write: the new id of a creation or a rename, `null` when the write is queued |
| `ids` | several items, in the order acted on |
| `count` | how many items the write matched |
| `queued` | `true` when the write waits for a sync engine rather than being done; a boolean only, no queue id |
| `response` | a protocol reply, as the server sent it |
| `warnings` | warnings the server attached to an accepted write |
| `path` | a file the command wrote |

A binary shares receipt types across its commands (one mailbox write output for create, rename and delete) rather than one type per command; a generic map-of-fields receipt is not allowed, being a message with extra steps. An action with nothing to name still names its target (`{mailbox}` for an IMAP `SELECT`).

### One command, one shape, one schema

Every command that prints registers its schema under its command path. A command whose result varies prints one type whose alternatives are told apart by their keys. pimalaya-cli provides a check taking the clap command tree and the registry, and reporting every leaf with no key unless the binary declares it silent, and every key naming no leaf. Its toolkit commands (`completion`, `manual`, `json-schema`) register their own outputs.

### Shapes

- **Collections** are always printed, `[]` when empty, under a plural key naming their rows (`envelopes`, `cards`), never a generic one (`list`, `response`, `items` for messages, `ids` for objects). A `get` prints the row its `list` prints.
- **Absent values** are `null`, the key always present. An empty text is never an absent one: `""` means the value is the empty string. Provider payloads keep their own omissions.
- **Paging** never truncates silently. A command that can stop before the end prints `nextPage`, `null` once complete, and takes it back through `--page-token`, whatever the backend's own token or link looks like. `--page N` may stay for humans.
- **Values** a binary defines (statuses, actions, kinds, error codes) are kebab-case. Values a protocol defines keep its spelling (iCalendar `NEEDS-ACTION`, Gmail `labelShow`).
- **Times**: an instant the binary resolves is RFC 3339 in UTC with `Z`; a timestamp read from content keeps its own offset; a date is `YYYY-MM-DD`; a duration is integer seconds. No epoch numbers in a binary's own types.
- **Bytes** under `--json` are never written raw to stdout and never decoded lossily. Binary content is `{data, size, mime}`, `data` in base64. vCard and iCalendar text is a string field when it is valid UTF-8, base64 otherwise.
- **Protocol namespaces are raw-faithful**: a protocol command prints the server's payload as received, not re-serialized through a type that invents `null`s or drops fields, wrapped under a designed collection key. Designed shapes belong to the cross-protocol commands and to what a binary computes itself.

### Shared nouns

These keep one shape in every binary:

- an account row: `{name, default, backends}`;
- a backend check row: `{backend, ok, code, error}`;
- the wizard's output: `{name, default, document}`;
- a collection row: `{id, name, description, color, default}`, `name` never `displayName`;
- a queued action's row id: `row`;
- an address: `{name, email}`.

### Failures and outcomes

Exit 1 prints the failure object, and only it. The failure object prints `code` always, `null` when the chain holds none, beside `error`, `sources` and `backtrace`. A binary has one failure object; a refusal with details (the pimdir CLI's refused query) carries them in a coded failure, not in a second shape. A failure folded into a result keeps its code: each `check` row carries `code`.

A command that ran to its end but whose outcome is not a full success prints its result and exits 2 or above, each code's meaning documented by the command (neverest's conflicted and incomplete runs today). A failed `check` prints its whole report, every row, and exits 2.

### Events

A command MAY print events before its result: what happens while it runs, as data.

- An event is an object whose root carries `event`, a kebab-case name. A result object never has an `event` key at its root, which the registry check enforces. A caller tells each line apart as it reads it.
- The last line is the outcome: the failure object when the exit code is 1, the result otherwise. Every line before it is an event. A command printing only events (a watch) has no result.
- Events are data, never failures. A failed item inside a batch is an event field; the command's own failure is the failure object.
- Under `--json`, events are one line each, flushed. In text mode they are their `Display`, written to stdout when stdout is a terminal and dropped when it is not, so `cmd > file` captures the result alone; a binary may log them instead.
- The registry holds an `events` map beside `schemas`. A command printing events registers there, under its command key, one type: a `oneOf` of its tagged event objects. `json-schema` writes it as `<key>.events.json`, its `$id` `https://pimalaya.org/schemas/<bin>/v<version>/<key>.events.json`. A command without events has no entry, a watch has events and no result entry.
- MCP servers relay events as progress notifications and the result as the tool result; a REPL prints each command's events, then its result.

The printer gains `event(value)`, writing an event as above, beside `out(value)`, which writes the result.

## Decided (2026-11-03)

- **No `Message` at all**, rather than a schema'd `Message` kept for bare confirmations: every action names something, and a missing type is enforceable where a discouraged one is not.
- **`queued: true` on receipts.** Until now the family's specs kept queued writes out of JSON and in the confirmation prose; without prose, that state would vanish from JSON entirely. The boolean is backend-neutral and carries no pimdir detail.
- **Absent is `null`**: schemas can mark every key required, and MCP structured output keeps one key set. It reaches the failure object, whose `code` was the one omitted key of the contract.
- **Protocol namespaces are raw-faithful**, cardamum's existing rule, over designed shapes (himalaya Gmail, calendula Google Calendar).
- **A queue row is `row`**, the name cardamum, calendula and unfog use and the column himalaya's queue view shows.
- **Text events only on a terminal**, so a redirect never mixes progress into data.

## Compatibility

Breaking for every binary: payload shapes change and pimalaya-cli drops `Message`. pimalaya-cli lands it in 0.3.0, which is pending. Each binary lands its own change, listing its exceptions and leftovers from the 2026-11-03 review, at a breaking release: himalaya 3.0.0, ortie and comodoro 3.0.0 (already owed for camelCase keys), the 0.x binaries at their next minor. Adding events to a command that already exists is breaking too, since a caller filtering the whole output meets lines it did not expect.

## Scope / non-goals

pimalaya-cli owns the mechanism (no `Message`, the event printer, the page token argument and the `nextPage` wrapper, the bytes type, the registry maps and checks, the failure object) and these rules. The shapes of a binary's own nouns (himalaya flags, envelopes, cardamum cards) belong to its own change. Text-mode wording is each binary's.
