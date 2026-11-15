---
cairn: spec
capability: commands
status: current
---

# Commands

What the command trees of the binaries share, so a person or a script moving from one binary to another finds the same commands and the same verbs. Each binary owns its commands; pimalaya-cli ships what they print (cairn/spec/terminal.md, cairn/spec/output.md).

### Requirement: Every binary carries an account group
Every binary SHALL carry an `account` group holding `account list`, printing the accounts its configuration declares without reaching anything, and `account check`, opening what each account stands on (each backend it declares, for a binary with backends) and printing one row each. A binary with backends SHALL also carry `account capability`. A binary that had a top-level `check` SHALL keep it as a hidden alias of `account check`.

#### Scenario: A binary moving its check
- GIVEN a binary whose check ran as `check`
- WHEN it takes the account group
- THEN `account check` runs the check, and `check` still does without being listed in the help

### Requirement: A verb keeps one meaning
`add` SHALL create an item and `create` a collection; `search` SHALL run a query and `list` walk a collection. A binary that spelled one of them otherwise SHALL keep the old verb as a hidden alias. Two pairs SHALL stay, each fitting its domain: `mailbox rename` beside `addressbook update` and `calendar update`, IMAP renaming a mailbox and nothing else where an address book or a calendar has a name, a description and a colour to change; and `raw`, the namespace command of a line protocol (IMAP, SMTP, ManageSieve) sending its commands as typed, beside `request`, the one of an HTTP API (JMAP, Graph, People) sending one request.

#### Scenario: Creating a card
- GIVEN a contacts binary that spelled the creation of a card `card create`
- WHEN it takes the family verbs
- THEN `card add` creates the card, and `card create` still does without being listed in the help
