---
cairn: delta
change: tools-are-gated-by-what-they-do
---

## ADDED Requirements

### Requirement: Gates list what a tool may do
A server started without a gate flag SHALL list only the tools that read. `--allow-write` SHALL list the tools that change the account's data, and `--allow-send` the tools that act beyond the account, such as sending mail, an invitation or an auto-reply. A binary MAY add a gate of its own for a narrower destructive class, as ortie's `--allow-revoke` for revoking a token at its authorization server. An argument that makes a listed tool write or send SHALL be listed only under the gate of what it does. A tool or an argument a gate leaves out SHALL be refused as any tool or argument not listed.

#### Scenario: No gate
- GIVEN a binary whose tools read a message, with a `seen` argument flagging it, and send one
- WHEN its server starts without a gate flag
- THEN `tools/list` lists the reading tool alone, without `seen`

### Requirement: Annotations describe the tool as listed
A tool's `readOnlyHint` SHALL be `true` exactly when the tool, with the arguments it lists, changes no data. Its `openWorldHint` SHALL be `true` exactly when it acts beyond the account's own data, reaching other people (sending mail, an invitation, an auto-reply); it SHALL NOT mean that the tool reaches the network, which every remote backend does. A gate SHALL move neither hint by itself: a tool listing the same arguments SHALL carry the same hints whichever gates are open.

#### Scenario: A remote listing and an invitation
- GIVEN a tool listing the events of a remote calendar and a tool inviting attendees
- WHEN the tools are listed
- THEN the first carries `readOnlyHint` `true` and `openWorldHint` `false`, the second `readOnlyHint` `false` and `openWorldHint` `true`

## MODIFIED Requirements

## REMOVED Requirements
