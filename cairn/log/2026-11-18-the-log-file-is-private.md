---
cairn: log
change: the-log-file-is-private
landed: 2026-11-18
---

# The log file is private

Finding KIT-4 of the CLI family analysis of 2026-11-15, its file half.

## What landed

- `file::private_options` (src/file.rs): `OpenOptions` with the mode 0600 and `O_NOFOLLOW` on Unix, plain elsewhere, for the apps' private files too (himalaya's mbox cache, HIM-10; the composer drafts of cardamum and calendula, CAR-14 and CAL-13).
- `Logger::try_init` (src/log.rs) opens `--log-file` through it in append mode, with the context `Cannot open log file <path>`. The `--log-file` help says so.
- libc 0.2 is a direct dependency on Unix under `terminal` (no feature), already in the tree through dirs-sys.
- `a_new_log_file_is_private`, `a_log_file_never_follows_a_symlink` and `an_unopenable_log_file_is_named`, each failing before the fix (0644, the target created through the link, `No such file or directory (os error 2)`).

Additive: the `file` module is new. Still open from KIT-4: log lines carry server strings unsanitized.

## Capabilities moved

- [terminal](../spec/terminal.md): *A private file is its owner's* and *The log file is private* added.
