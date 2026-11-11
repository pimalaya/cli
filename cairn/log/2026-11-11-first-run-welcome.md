---
cairn: log
change: first-run-welcome
landed: 2026-11-11
---

# First-run welcome

Ortie 3.0's first-run code moved into pimalaya-cli behind `terminal`, generalised over the binary name and the app's sentences.

- src/shell.rs: `quote(s)` is Ortie's `shell_quote`, `invocation(bin, config_paths, account)` its `invocation` with the binary name first. Ortie's `shell_quote_escapes_embedded_quotes` and `invocation_carries_the_account_and_config` ported as `quote_escapes_embedded_quotes` and `invocation_carries_the_account_and_config`.
- src/welcome.rs: `Welcome { bin, intro, path, config_paths, sample, sample_url, next, command }`, whose `Display` writes Ortie's `welcome` layout; `shipped_sample(bin)` and the private `find_sample(dir, bin)`, Ortie's lookup of share/doc/<bin>/config.sample.toml; `is_wanted(printer)`, Ortie's main.rs guard (not JSON, stderr a terminal). The welcome does not look the sample up itself, so it stays a pure rendering: the app passes `shipped_sample(bin)`. An empty `command` prints the invocation alone, with no trailing space.

A scratch test, not kept, compared `Welcome` with a verbatim copy of Ortie's `welcome` for Ortie's sentences and URL over 8 inputs (a missing parent directory, an existing one, a relative path, no `-c`, two `-c`, each with and without a sample): byte-identical. The kept tests use a neutral `app`: `the_welcome_reads_in_paragraphs` pins the whole text, `the_welcome_commands_run_as_printed` and its Windows twin `the_welcome_commands_run_in_cmd_and_powershell` are Ortie's, and `the_shipped_sample_is_found_installed_or_extracted` is Ortie's on a process-unique directory under the system temp dir rather than tempfile, which pimalaya-cli does not depend on.

Spec: terminal ("A printed command runs as pasted", "A missing configuration meets a welcome" and "The shipped sample is found next to the binary" added).
