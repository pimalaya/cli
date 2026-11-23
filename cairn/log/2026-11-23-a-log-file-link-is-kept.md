---
cairn: log
change: a-log-file-link-is-kept
landed: 2026-11-23
---

# A log file link is kept

Found by the live test night of 2026-11-23 (.lanes/live/journal.md, carillon and cardamum agents): `--log-file lnk.log`, `lnk.log` a symbolic link to an existing file, exited 0 and wrote into the link's target, though the help and the spec say a link is refused. `path_parser` canonicalized the path, resolving the link before `open_log_file` opened it `O_NOFOLLOW`. A dangling link was already refused, canonicalization failing on it.

## What landed

- src/clap/parsers.rs: `expanded_path_parser` shell-expands without resolving; `path_parser` is it plus canonicalization.
- src/clap/args.rs: `LogFlags::file` parses through `expanded_path_parser`.
- src/log.rs: `a_log_file_flag_never_follows_a_symlink_to_a_file` parses `--log-file` on a link to an existing file and opens it: refused; it opened the target before the fix.

## Spec

[terminal](../spec/terminal.md): scenario *A link to an existing file at the log path* under *The log file is private*.
