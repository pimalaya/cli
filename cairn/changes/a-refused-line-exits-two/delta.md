---
cairn: delta
change: a-refused-line-exits-two
---

## ADDED Requirements

### Requirement: A refused command line exits 2
pimalaya-cli SHALL provide `clap::parse`, which a binary calls in place of clap's `Parser::parse`. Help and version SHALL print and exit as clap does. A command line the grammar refuses (an unknown flag or command, a missing argument, a bad value) SHALL exit 2; when a `--json` word comes before any `--`, it SHALL print on stdout the failure object coded `usage-invalid`, its `error` being clap's message without its `error: ` prefix or styles, and clap's rendering on stderr otherwise. pimalaya-cli SHALL provide `error::usage_error`, building that coded failure from a clap error, for every other place a binary turns a clap error into a failure.

#### Scenario: A mistyped flag under --json
- GIVEN a binary parsing its arguments with `clap::parse`
- WHEN it runs with `--json --frobnicate`
- THEN stdout holds one failure object with `"code":"usage-invalid"` and an `error` naming `--frobnicate`, and the exit code is 2

## MODIFIED Requirements

### Requirement: Failures carry an optional stable code
The error report SHALL print a `code` field beside `error`, `sources` and `backtrace`: the code of the outermost coded error in the chain, `null` when the chain holds none. Code values SHALL be kebab-case. The code vocabulary SHALL belong to each binary; pimalaya-cli SHALL provide only the mechanism, and the codes it raises itself, `usage-invalid` alone, which the failure object's schema admits beside the binary's. A binary SHALL have one failure object, and a failure folded into a result SHALL keep its code.

### Requirement: A protocol refusal is a result
Inside a protocol namespace, a completion the server refuses (an IMAP `NO` or `BAD`, an SMTP 4xx or 5xx reply, a ManageSieve `NO`, an HTTP 4xx or 5xx carrying the provider's error body) SHALL print as the result and exit 3. A command that gets no completion (a network, TLS or decoding failure, a local error) SHALL print the failure object and exit 1. Outside protocol namespaces, a refusal SHALL be a coded failure.

### Requirement: A partial outcome prints its result
A command that ran to its end with an outcome that is not a full success SHALL print its result and exit with a code of 3 or above, whose meaning the command documents; 2 SHALL stay clap's, for a refused command line. A `check` with a failed backend SHALL print every row and exit 3. pimalaya-cli SHALL provide a marker error a command returns once its result is printed, which the error report turns into the exit status without printing anything more.

### Requirement: Each line answers through its own printer
Each line SHALL print its result through a printer of its own, JSON when the session's `--json` or the line's is set, the following lines keeping the session's format. The loop SHALL add nothing after a result, every text output ending with its own newline, and SHALL flush stdout after each line. A failed command SHALL print the failure object of the binary's error codes through that printer on stdout, whether stdin is a terminal or not, its text form hinting `Restart the session with --log-level to enable more verbose logs` while debug logs are off, since a line cannot set `--log-level`. A command ending with a partial outcome SHALL print nothing after its result. A line clap cannot parse SHALL print clap's rendering on stderr when stdin is a terminal, and otherwise the failure object coded `usage-invalid` on stdout, its message clap's without its `error: ` prefix. When stdin is a terminal, the prompt `<bin>> ` SHALL render on stderr before each line.

### Requirement: A call is checked before it runs
`tools/call` SHALL answer -32602 when `params` is not an object, when `name` is missing or names no listed tool, and when `arguments` is neither an object nor absent or `null`. When the tool's `inputSchema` sets `additionalProperties` to `false`, an argument it does not declare SHALL be refused as a tool result with `isError: true` and the failure object coded `usage-invalid`, `Unknown argument <key>, <tool> takes <properties>`, or `takes none`. Any other call SHALL be handed to the binary with its arguments as an object.

### Requirement: A tool's input is its command's grammar
`mcp::grammar::Grammar` SHALL describe a built clap command as one property per argument, named after its long flag or the lowercased value name of a positional, leaving out global, hidden, help and version arguments, arguments taking a path (a path value hint or a `PATH` value name), the ones a binary filters out, and the ones its fixed words set. A flag SHALL be a boolean, a repeatable argument an array, an argument the binary takes one value of and any other argument a scalar; a value SHALL be an `integer` when its value parser yields a Rust integer type and a `string` otherwise, possible values an `enum`. The help SHALL be the `description`, unless the binary replaces it, and a long flag of a property written between backquotes in it SHALL read as the property's name. Required arguments, repeatable positionals needing at least one value and arguments taking one value SHALL be `required`, an optional positional taking one value SHALL NOT, and the schema SHALL set `additionalProperties: false`. `argv` SHALL write the command path, `--<long>=<value>` for each option value, `--<long>` for a true flag, the fixed words, then `--` and the positionals in their order. It SHALL take a lone value where an array is expected as an array of one, and an array of one for an argument taking one value, and SHALL refuse an undeclared key, a value of the wrong type and a count other than one for an argument taking one, each a failure coded `usage-invalid`.
