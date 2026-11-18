---
cairn: tasks
change: the-log-file-is-private
---

# Tasks

- [x] src/file.rs: `private_options`, 0600 and `O_NOFOLLOW` on Unix
- [x] src/log.rs: open `--log-file` through it, naming the path on failure
- [x] Cargo.toml: libc on Unix under `terminal`
- [x] Tests: a new log file is 0600, a symbolic link at the path is refused without creating its target, an unopenable path is named (each failed before the fix)
- [x] CHANGELOG; fold into the spec; log; land
