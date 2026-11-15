---
cairn: delta
change: the-rules-match-the-family
---

## ADDED Requirements

## MODIFIED Requirements

### Requirement: Every binary carries an account group
A binary with a unified API SHALL also carry `account capability`. A binary whose backends serve no unified API carries none, having no capability for it to report.

### Requirement: A write answers a receipt
`id` SHALL be the item as it is after the write. For a queued write it SHALL be the id the store gives the item, its link id, and `null` only when the backend gives none; a queued write SHALL NOT print a placeholder id (`q<id>`, `0`, `""`).

### Requirement: Bytes are never raw under JSON
vCard and iCalendar content SHALL be a string beside `encoding`: the text when its bytes are valid UTF-8, `encoding` then `utf-8`, and their base64 (RFC 4648 standard alphabet, padded) otherwise, `encoding` then `base64`.

## REMOVED Requirements
