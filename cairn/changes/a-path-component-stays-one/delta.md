---
cairn: delta
change: a-path-component-stays-one
---

## MODIFIED Requirements

### Requirement: Untrusted text is defused
`sanitize::path_component` SHALL turn a name into one path component by the rule of pimdir STORAGE §18, as io-pimdir's `security::path_component` does: every `/` and `\`, NUL, control character and bidirectional formatting character (U+061C, U+200E, U+200F, U+202A to U+202E, U+2066 to U+2069) replaced by `_`, and an empty name written `_`, `.` and `..` written `_` and `__`. It SHALL be available with the `terminal` feature, as `sanitize::text` and `sanitize::lines` are. (The rest of the requirement is unchanged.)

#### Scenario: An attachment name climbing out of its directory
- GIVEN the attachment names `../../etc/passwd`, `..` and `invoice\u{202e}fdp.exe`
- WHEN each goes through `sanitize::path_component`
- THEN they read `.._.._etc_passwd`, `__` and `invoice_fdp.exe`

## REMOVED Requirements
