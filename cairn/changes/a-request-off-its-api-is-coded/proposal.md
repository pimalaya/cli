---
cairn: change
id: a-request-off-its-api-is-coded
status: landed
created: 2026-11-19
---

# A request off its API is coded

## Why

X16 of the CLI alignment plan (.github cairn/changes/cli-alignment-2026-11-15) made every raw `request` command refuse a full URL off its provider's API origin, so the bearer token never goes to another host. X18 found that refusal uncoded in all three apps, each worded its own way: himalaya src/shared/raw_json.rs:24 (`Refused URL ...: it leaves the API at ...`), cardamum src/msgraph/request.rs:68 and src/gpeople/request.rs:69 (`Cannot send a request to ...: it is off the API origin ...`), calendula src/shared/raw_json.rs:28 (`Cannot send a <provider> request to ...: a full URL must be on ...`). A code is a contract scripts read, so the owner ruled on 2026-11-19 for one shared code, `request-origin`, in pimalaya-cli's shared table, before the app majors.

## What

- `CommonCode::RequestOrigin`, serialized `request-origin`, beside `usage-invalid`; the failure object's schema admits it in every binary. It exits 1.
- `error::request_origin(url, api)`, the coded refusal from the two origins, with one message for the family.
- `CommonCode` becomes `#[non_exhaustive]`, so a later shared code is additive.

Breaking by semver for a crate matching `CommonCode` exhaustively; no app does (ortie and calendula only build `UsageInvalid`).
