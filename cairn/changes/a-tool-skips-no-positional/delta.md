---
cairn: delta
change: a-tool-skips-no-positional
---

## ADDED Requirements

### Requirement: A tool's command skips no positional
`mcp::grammar::Grammar::new` SHALL panic on a command set with clap's `allow_missing_positional`, naming the tool, since its `argv` writes the positionals after `--`, where clap no longer skips a missing positional of such a command. A binary SHALL take an optional value standing before a required positional as a flag instead.

#### Scenario: An optional positional before a required one
- GIVEN the command `list [CALENDAR] <ID>` set with `allow_missing_positional`
- WHEN a binary builds its grammar
- THEN it panics with a message naming `allow_missing_positional`, before any tool is listed

## MODIFIED Requirements

## REMOVED Requirements
