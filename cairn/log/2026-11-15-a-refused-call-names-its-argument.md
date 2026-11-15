---
cairn: log
change: a-refused-call-names-its-argument
landed: 2026-11-15
---

# A refused call names its argument

Task L5 of the CLI alignment round of 2026-11-15, from a runtime MCP report.

## What landed

- `mcp::grammar::Grammar::argv` refuses a required argument missing or `null` once the values given are checked: ``Missing argument `card-id` ``, coded `usage-invalid`, ortie's wording (src/mcp.rs:779). Binaries get it with no change, the check running before clap.
- `mcp::grammar::Grammar::usage_error`, turning clap's refusal of a call's argv into ``Missing argument `<name>` `` or ``Invalid value `<value>` for `<name>`: <reason>``, clap's message for any other refusal; the binaries call it where they call `error::usage_error` today.
- The `mcp` feature enables clap's `error-context`, which himalaya, cardamum, calendula and ortie enable already.
- `a_missing_argument_is_named_as_the_input_names_it`, `a_refused_value_names_its_property_and_clap_reason`, `a_refusal_naming_no_property_keeps_clap_message`; two argv tests now give the required arguments they left out.

## Capabilities moved

- [mcp](../spec/mcp.md): *A refused call names its arguments* added; *A tool's input is its command's grammar* modified.
