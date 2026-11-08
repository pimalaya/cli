---
cairn: change
id: a-password-is-typed-once
status: landed
created: 2026-11-08
---

# A password is typed once

## Why

The account wizards of himalaya, cardamum, calendula and neverest ask for a secret through `wizard::keyring::prompt_secret`: a strategy pick list, then a keyring entry name. The picker only records a read command, so the user must have stored the password under that entry beforehand, in another terminal, or the connection test that follows fails. Nothing tells the user which server receives the password, and nothing lists what the wizard picked before it saves.

The Pimalaya Android app's onboarding settled a better flow: the password is typed once, tested against the servers, then stored by the app itself; each sign-in names the endpoints the password goes to and flags one outside the address's domain; a review lists what was picked per domain; OAuth runs one sign-in. The CLIs should follow it, with Ortie (the Pimalaya OAuth broker CLI) as their OAuth sign-in.

## What

pimalaya-cli provides the shared pieces, behind the `wizard` feature; each CLI adopts them in its own change.

- `KeyringProvider::store` stores a secret through the provider's CLI, writing it on the program's stdin and never on its arguments, which any process can read. The entry is the one `read_command(None, key)` reads.
- `keyring::prompt_password` names every endpoint the password goes to, flags one whose host is outside the address's domain, then asks for the password once, masked and unconfirmed: the caller's connection test confirms it.
- `keyring::prompt_store`, called once the caller's test passed, offers the installed keyring providers, a custom command and a raw value, first entry by default. Picking a provider stores the typed password and reads it back; a provider that fails either step is reported and dropped from the list.
- `review::prompt` lists one line per picked domain and answers whether to continue or which row to change.
- `ortie::sign_in` runs `ortie configure` then `ortie auth get` for one account, Ortie's output going to stderr so a caller's redirected stdout keeps only the configuration, and answers the token read command for the configuration.

`prompt_secret` and `prompt_token` stay for custom paths. `KeyringProvider::write_command` stays as Ortie uses it, though its macOS form puts the secret on argv; the new store path does not.

Out of scope: the CLIs adopting these pieces, and Ortie's side of the contract (`ortie configure [INPUT] [--service SERVICE]... [--name NAME]`), built in parallel in the ortie repository.
