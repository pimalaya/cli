---
cairn: log
change: a-password-is-typed-once
landed: 2026-11-08
---

# A password is typed once

The CLI wizards asked for a keyring entry and only recorded its read command, so a password not stored beforehand failed the connection test. pimalaya-cli now has the pieces for the Android app's flow, behind the `wizard` feature; the CLIs adopt them in their own changes.

`KeyringProvider::store` writes the secret on the store program's stdin: `secret-tool store --label <key> account <key>`, `kwallet-query -w <key> kdewallet`, `pass insert -m -f <key>`, and for the macOS keychain `security -i` fed one `add-generic-password -U -s <key> -a <key> -w "<secret>"` line, `"` and `\` escaped and a line break refused (it would start a second command). Each entry is the one `read_command(None, key)` reads. A failure carries the program's stderr collapsed to one line, or `<program> failed, <status>` when it printed nothing. `write_command` is unchanged, Ortie still using it, its macOS form still putting the secret on argv through `$(cat)`.

`keyring::prompt_password` prints `Your password goes to <endpoint>` per endpoint, adding `, not on <domain>` when the host is outside the address's domain (hand-parsed, userinfo up to the last `@`, IPv6 brackets, trailing dot and case ignored), then one masked prompt. `keyring::prompt_store` offers the installed providers, `Custom shell command printing it` and `Raw in the configuration (plaintext, NOT recommended)`; a provider is stored to, read back and compared (one trailing newline aside), and a failure prints `Cannot store <label> in <provider>: <reason>` and drops it from the list. `review::prompt` pads row names to the widest and asks `What next?` with `Continue` then `Change <name>`. `ortie::sign_in` runs `ortie configure --name <name> --service <s>... -- <input>` then `ortie auth get -a <name>`, Ortie's stdout on a duplicate of this process's stderr, and answers `ortie token show -a <name>`; the input goes after `--` so an address starting with a dash stays positional.

Verified: 42 unit tests pass (5 new: store argv per provider, the `security -i` escaping and its line-break refusal, the off-domain rule, the `ortie configure` argv); clippy clean with all features; `--no-default-features` and `--features wizard` build; missing-docs and doc-link checks pass. A throwaway crate in a scratch directory ran `store`, the three prompts in a pseudo-terminal and `sign_in` against fake `pass`, `secret-tool`, `kwallet-query`, `security` and `ortie` scripts on a scratch `PATH`: the stdin held exactly the secret, a store that kept nothing or kept another value was reported and dropped, the raw entry asked nothing more, and with stdout redirected to a file the file held only the caller's output.

Unverified: the store commands against the real programs (never run on this machine), `security -i` quoting and exit status on macOS (the read-back catches a store that did not take), and the Windows branch of the stderr duplicate (the devshell has no Windows std).

Spec: wizard (new capability, 6 requirements added).

Left: himalaya, cardamum, calendula and neverest adopting the prompts in their own changes; Ortie's `configure [INPUT] [--service SERVICE]... [--name NAME]`, built in the ortie repository.
