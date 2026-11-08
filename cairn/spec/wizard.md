---
cairn: spec
capability: wizard
status: current
---

# Wizard

The account wizards of the Pimalaya CLIs share their secret, OAuth and review steps through pimalaya-cli, behind the `wizard` feature. A password is typed once, tested by the caller, then stored by the wizard; the user sees where it goes and reviews every pick before saving.

### Requirement: A secret is stored through stdin
`KeyringProvider::store` SHALL store a secret at an entry through the provider's CLI, writing the secret on the program's stdin and never on its arguments. The stored entry SHALL be the one the provider's `read_command(None, key)` reads, and an entry already at that key SHALL be replaced. A provider whose CLI takes a secret only as an argument SHALL be run in its interactive mode, the command line, secret included, written on stdin with quotes and backslashes escaped; a secret or key holding a line break SHALL be refused there before anything runs. A program exiting unsuccessfully SHALL fail the store with its stderr text, never with the secret.

#### Scenario: A password stored in pass
- GIVEN the entry `example-imap` and a typed password
- WHEN the password is stored in pass
- THEN `pass insert -m -f example-imap` runs with the password on its stdin, and `pass show example-imap` reads it back

#### Scenario: A password holding a quote stored in the keychain
- GIVEN a password holding `"` and `\`
- WHEN it is stored through the keychain CLI
- THEN `security -i` runs with no secret in its arguments, and its stdin holds one `add-generic-password` line with the password double-quoted, each `"` and `\` escaped by a backslash

### Requirement: The user sees where a password goes
`keyring::prompt_password` SHALL print on stderr, before asking, one line per endpoint the secret goes to, naming it after the prompt's label (`Password` reads `Your password`, an initialism such as `API token` stays as written). When the address's domain is given and an endpoint's host is neither that domain nor one of its subdomains, compared case-insensitively, the line SHALL say the endpoint is not on that domain. A host is read from `scheme://[userinfo@]host[:port][/path]`, the userinfo ending at the last `@`.

#### Scenario: Mail hosted elsewhere
- GIVEN the domain `example.org` and the endpoint `imaps://imap.example.net:993`
- WHEN the password prompt shows
- THEN stderr reads `Your password goes to imaps://imap.example.net:993, not on example.org`

#### Scenario: A lookalike host
- GIVEN the domain `example.org` and the endpoints `imaps://example.org.example.net` and `imaps://notexample.org`
- WHEN the password prompt shows
- THEN both lines say they are not on `example.org`, while `imaps://IMAP.Example.org:993` would not

### Requirement: A password is typed once
`keyring::prompt_password` SHALL ask for the password once, masked, with no confirmation prompt: the caller's connection test confirms it.

### Requirement: The wizard stores the tested password
`keyring::prompt_store` SHALL offer, first entry selected, each installed keyring provider in the order `KeyringProvider::available` gives, then a custom shell command, then the raw value in the configuration. Picking a provider SHALL store the already-typed password, read it back with the provider's read command, and answer that read command when the value read matches, one trailing newline aside. A store or read that fails, or a value that does not match, SHALL print one line on stderr naming the provider and the reason, no secret in it, and SHALL offer the list again without that provider. The custom entry SHALL ask for the command and answer it as a shell line; the raw entry SHALL answer the typed password without asking again.

#### Scenario: A keyring that stores nothing
- GIVEN an installed provider whose CLI exits 0 but keeps nothing
- WHEN the user picks it
- THEN stderr names the provider and says the read failed or did not match, and the list shows again without it

#### Scenario: Raw storage
- GIVEN a typed password
- WHEN the user picks the raw entry
- THEN the answer is that password, with no second password prompt

### Requirement: The review lists each pick
`review::prompt` SHALL print on stderr one row per picked domain, its name padded to the widest name, then offer `Continue`, selected, and one `Change <name>` entry per row, the name lowercased, answering continue or the index of the row to change.

#### Scenario: Mail and contacts picked
- GIVEN the rows `Mail` and `Contacts`
- WHEN the review shows
- THEN stderr reads `Mail      ...` and `Contacts  ...`, and the list offers `Continue`, `Change mail`, `Change contacts`

### Requirement: OAuth hands off to Ortie
`ortie::sign_in` SHALL fail as not installed when `ortie` is not on the `PATH`. Otherwise it SHALL run `ortie configure` with `--name`, one `--service` per service and the address after `--`, then `ortie auth get -a <name>`, each with stdin and stderr inherited and stdout sent to this process's stderr, and SHALL fail naming the command and its exit status when one exits unsuccessfully. On success it SHALL answer `ortie token show -a <name>`.

#### Scenario: A configuration redirected to a file
- GIVEN `himalaya configure > config.toml` choosing an OAuth sign-in
- WHEN Ortie prompts and prints during `configure` and `auth get`
- THEN all of it reaches the terminal through stderr, and config.toml holds only what the caller prints
