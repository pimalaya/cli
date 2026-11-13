---
cairn: delta
change: the-tree-gets-its-pages
---

## ADDED Requirements

### Requirement: Every command has a manual page
`manual` SHALL build the command tree with clap's `help` subcommands disabled, then give every visible command at any depth a page named after its dashed path, as in `<bin>-envelope-list`, its `.TH` title that name and its source the root's name and version, its synopsis the whole command. A name given SHALL be one of those, and every page listed under a page's SEE ALSO SHALL be one the same run can write. The `manual` and `json-schema` help SHALL name no program's command as an example.

#### Scenario: A leaf page
- GIVEN the tree `test` with the subcommand `sub` holding `list` and `search`
- WHEN `manual test-sub-list` runs
- THEN the page is titled `.TH test-sub-list 1` and its synopsis reads `test sub list`

#### Scenario: Every page
- GIVEN the same tree with a hidden subcommand `secret`
- WHEN `manual --dir <dir>` runs
- THEN it writes `test.1`, `test-sub.1`, `test-sub-list.1` and `test-sub-search.1`, and `test-sub.1` lists `test-sub-list(1)` and `test-sub-search(1)`, no `help(1)`

## MODIFIED Requirements

### Requirement: A tool's input is its command's grammar
`mcp::grammar::Grammar` SHALL describe a built clap command as one property per argument, named after its long flag or the lowercased value name of a positional, leaving out global, hidden, help and version arguments, arguments taking a path (a path value hint or a `PATH` value name), the ones a binary filters out, and the ones its fixed words set. A flag SHALL be a boolean, a repeatable argument an array, an argument the binary takes one value of and any other argument a scalar; a value SHALL be an `integer` when its value parser yields a Rust integer type and a `string` otherwise, possible values an `enum`. The help SHALL be the `description`, unless the binary replaces it, and a long flag of a property written between backquotes in it SHALL read as the property's name. Required arguments, repeatable positionals needing at least one value and arguments taking one value SHALL be `required`, an optional positional taking one value SHALL NOT, and the schema SHALL set `additionalProperties: false`. `argv` SHALL write the command path, `--<long>=<value>` for each option value, `--<long>` for a true flag, the fixed words, then `--` and the positionals in their order. It SHALL take a lone value where an array is expected as an array of one, and an array of one for an argument taking one value, and SHALL refuse an undeclared key, a value of the wrong type and a count other than one for an argument taking one.

#### Scenario: A value starting with a dash
- GIVEN the command `attachment download` with `--mailbox`, the flag `--stdout` fixed, and the positionals `message-id` and `attachment-id`, the latter taking one value
- WHEN the arguments `mailbox` `-odd`, `message-id` `m2` and `attachment-id` `"3"` are turned into an argv
- THEN it reads `attachment download --mailbox=-odd --stdout -- m2 3`, and `stdout` is no property

#### Scenario: A query given as one string
- GIVEN a command whose positional `query` takes many words
- WHEN the argument `query` is `"subject hello order by date desc"`
- THEN the argv carries it as one word after `--`, as `["subject hello order by date desc"]` would

#### Scenario: A page number
- GIVEN an option `--page` parsed by `value_parser!(u32).range(1..)`
- WHEN the input schema is built
- THEN `page` is typed `integer`

## REMOVED Requirements
