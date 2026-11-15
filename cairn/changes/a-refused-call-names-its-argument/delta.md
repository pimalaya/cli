---
cairn: delta
change: a-refused-call-names-its-argument
---

## ADDED Requirements

### Requirement: A refused call names its arguments
`Grammar::usage_error` SHALL turn clap's refusal of a call's argv into a failure coded `usage-invalid` naming each argument as the tool's input does, never clap's usage text: a missing argument as ``Missing argument `<name>` `` (``Missing arguments `<a>`, `<b>` `` for several), a refused value as ``Invalid value `<value>` for `<name>`: <reason>``, the reason being the value parser's message or ``the possible values are `<a>`, `<b>` ``. A refusal of another kind (a conflict between arguments), or naming an argument that is no property, SHALL keep clap's message as `error::usage_error` words it.

#### Scenario: A page that is no number
- GIVEN a tool whose `page` argument is parsed by `value_parser!(u32).range(1..)`
- WHEN a call gives `page` `"x"` and clap refuses its argv
- THEN the tool result is an error whose failure object is coded `usage-invalid` and reads ``Invalid value `x` for `page`: invalid digit found in string``

## MODIFIED Requirements

### Requirement: A tool's input is its command's grammar
`mcp::grammar::Grammar` SHALL describe a built clap command as one property per argument, named after its long flag or the lowercased value name of a positional, leaving out global, hidden, help and version arguments, arguments taking a path (a path value hint or a `PATH` value name), the ones a binary filters out, and the ones its fixed words set. A flag SHALL be a boolean, a repeatable argument an array, an argument the binary takes one value of and any other argument a scalar; a value SHALL be an `integer` when its value parser yields a Rust integer type and a `string` otherwise, possible values an `enum`. The help SHALL be the `description`, unless the binary replaces it, and a long flag of a property written between backquotes in it SHALL read as the property's name. Required arguments, repeatable positionals needing at least one value and arguments taking one value SHALL be `required`, an optional positional taking one value SHALL NOT, and the schema SHALL set `additionalProperties: false`. `argv` SHALL write the command path, `--<long>=<value>` for each option value, `--<long>` for a true flag, the fixed words, then `--` and the positionals in their order. It SHALL take a lone value where an array is expected as an array of one, and an array of one for an argument taking one value, and SHALL refuse an undeclared key, a value of the wrong type, a count other than one for an argument taking one and, once the values given are checked, a required argument missing or `null`, as ``Missing argument `<name>` `` (``Missing arguments `<a>`, `<b>` `` for several), each a failure coded `usage-invalid`.

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
