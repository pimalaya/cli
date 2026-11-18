---
cairn: change
id: the-log-file-is-private
status: landed
created: 2026-11-18
---

# The log file is private

## Why

The CLI family analysis of 2026-11-15 (finding KIT-4, rechecked 2026-11-16) found `--log-file` created under the umask, 0644 under the usual 022, and opened through a symbolic link (src/log.rs:38), while a trace log holds protocol exchanges. Another user able to plant a link at the path gets the trace written where the link points. A log file that cannot be opened was reported as `No such file or directory (os error 2)`, naming no file. The same class reaches the apps: himalaya's mbox cache (HIM-10) and the composers' drafts of cardamum and calendula (CAR-14, CAL-13) are created under the umask too, so the kit gives them the helper.

## What

- `file::private_options`: `OpenOptions` which, on Unix, create a file at 0600 whatever the umask and refuse a symbolic link (`O_NOFOLLOW`); the caller adds its access mode. A file that exists keeps its permissions. Elsewhere they are plain options.
- `Logger::try_init` opens `--log-file` through it, in append mode, and names the path when it cannot: `Cannot open log file <path>`, the OS error as its source.
- libc becomes a direct dependency on Unix under `terminal`, for `O_NOFOLLOW`; it was in the tree already through dirs.

Left out: the logger still writes server strings unsanitized into the log (the rest of KIT-4), and private directories (HIM-10's cache directory) stay the apps' own.

Additive: the `file` module is new, no public item changed.
