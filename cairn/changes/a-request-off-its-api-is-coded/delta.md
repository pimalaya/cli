---
cairn: delta
change: a-request-off-its-api-is-coded
---

## ADDED Requirements

### Requirement: A request off its API is coded request-origin
A raw protocol request command (`msgraph request`, `gmail request`, `gcal request`, `gpeople request` and the like) SHALL refuse a URL whose origin (scheme, host and port) is not the origin of the API its bearer token is for, before anything is sent, with a failure coded `request-origin`, exiting 1. pimalaya-cli SHALL provide `error::request_origin(url, api)`, building that failure from the two origins as a URL serializes them, its message ``Cannot send a request to `<url>`: it is off the API origin `<api>`, which alone receives the account's token``. A binary SHALL NOT code that refusal with a code of its own vocabulary.

#### Scenario: A full URL on another host
- GIVEN a binary whose `msgraph request` sends the account's token to `https://graph.microsoft.com`
- WHEN it runs with `--json get https://evil.example/me`
- THEN stdout holds one failure object with `"code":"request-origin"`, the exit code is 1, and no request was sent

## MODIFIED Requirements

### Requirement: Failures carry an optional stable code
The code vocabulary SHALL belong to each binary; pimalaya-cli SHALL provide the mechanism and the family's shared codes, `CommonCode`, which the failure object's schema admits beside the binary's: `usage-invalid`, which it raises itself, and `request-origin`, which a binary raises. `error::with_code` leaves alone a chain already coded, by the binary or with a shared code, or holding a partial outcome. (The rest of the requirement is unchanged.)

## REMOVED Requirements
