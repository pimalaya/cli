---
cairn: delta
change: the-manual-names-the-version-once
---

## ADDED Requirements

## MODIFIED Requirements

### Requirement: Every command has a manual page
`manual` SHALL build the command tree with clap's `help` subcommands disabled, then give every visible command at any depth a page named after its dashed path, as in `<bin>-envelope-list`, its `.TH` title that name and its source the root's name and version, its synopsis the whole command. A page's VERSION section SHALL name the version once: a long version starting with `v` and a digit, as `long_version!` writes it, SHALL lose that `v` on the page, the page generator writing its own. A name given SHALL be one of those, and every page listed under a page's SEE ALSO SHALL be one the same run can write. The `manual` and `json-schema` help SHALL name no program's command as an example.

#### Scenario: A leaf page
- GIVEN the tree `test` with the subcommand `sub` holding `list` and `search`
- WHEN `manual test-sub-list` runs
- THEN the page is titled `.TH test-sub-list 1` and its synopsis reads `test sub list`

#### Scenario: Every page
- GIVEN the same tree with a hidden subcommand `secret`
- WHEN `manual --dir <dir>` runs
- THEN it writes `test.1`, `test-sub.1`, `test-sub-list.1` and `test-sub-search.1`, and `test-sub.1` lists `test-sub-list(1)` and `test-sub-search(1)`, no `help(1)`

#### Scenario: A long version written by long_version!
- GIVEN the tree `test` whose long version is `v1.2.3 +imap`
- WHEN `manual test` runs
- THEN the VERSION section reads `v1.2.3 +imap`, not `vv1.2.3 +imap`

## REMOVED Requirements
