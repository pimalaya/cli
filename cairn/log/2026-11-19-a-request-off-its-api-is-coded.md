---
cairn: log
change: a-request-off-its-api-is-coded
landed: 2026-11-19
---

# A request off its API is coded

Owner ruling of 2026-11-19 on X18 of the CLI alignment plan (the X16 origin refusal carried no code).

## What landed

- `CommonCode::RequestOrigin` (src/error.rs), serialized `request-origin`, beside `usage-invalid`, exiting 1; `CommonCode` is now `#[non_exhaustive]`.
- `error::request_origin(url, api)`, the coded refusal with the family's one message, ``Cannot send a request to `<url>`: it is off the API origin `<api>`, which alone receives the account's token``.
- `a_request_off_the_api_is_coded_request_origin`; `the_schema_admits_the_common_codes` checks `request-origin` too.

Breaking by semver only for an exhaustive `match` on `CommonCode`, which no app has.

## To rename in the apps, once pimalaya-cli is released

Each turns its plain `bail!` into `return Err(request_origin(&url.origin().ascii_serialization(), &base.origin().ascii_serialization()))`, adds a test asserting `code_of::<CommonCode>` is `RequestOrigin`, and names `request-origin` in its code table and its `request` command spec:

- himalaya: src/shared/raw_json.rs:24 (`api_url`, behind `gmail request` and `msgraph request`), its test at :65 matching the old text; cairn/spec/commands.md:67.
- cardamum: src/msgraph/request.rs:68 and src/gpeople/request.rs:69 (`resolve_url`), their tests at :112 and :109 matching `off the API origin` (still true of the shared message); cairn/spec/commands.md. Its `jmap request` posts to the session's `apiUrl` alone and has nothing to refuse.
- calendula: src/shared/raw_json.rs:28 (`request_url`, behind `gcal request` and `msgraph request`; the `provider` argument leaves the message); cairn/spec/commands.md.
- neverest, ortie, carillon: no `request` command and no origin refusal; nothing to rename.

## Capabilities moved

- [error](../spec/error.md): *A request off its API is coded request-origin* added; *Failures carry an optional stable code* modified.
