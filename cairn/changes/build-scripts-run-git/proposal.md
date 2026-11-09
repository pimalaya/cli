---
cairn: change
id: build-scripts-run-git
status: landed
created: 2026-11-09
---

# Build scripts run git

## Why

`build::git_envs` opens the repository with git2 to compute `GIT_DESCRIBE` and `GIT_REV` when the environment does not set them. git2 brings libgit2-sys and libz-sys, which build libgit2 and zlib from C through cc (or link the system's), so every consumer's build script compiles a C library and needs a C compiler to read two strings. Its `expect`s also panic when the repository opens but cannot be described, for instance one whose HEAD has no commit yet.

## What

- `git_envs` still forwards `GIT_DESCRIBE` and `GIT_REV` from the environment first.
- Otherwise it runs `git describe --all --always` (git2's `describe_all` with `show_commit_oid_as_fallback`) and `git rev-parse HEAD` (HEAD's commit id) through `std::process::Command`, from the build script's working directory, and trims their output.
- Both commands take `--git-dir=.git`, keeping the scope of git2's `Repository::open(".")`: the repository whose .git sits in the working directory, never a parent directory's. Plain `git` would search upward and describe, for example, a dotfiles repository around a crate source in ~/.cargo/registry.
- When git is missing, the directory holds no repository or a command fails, the variable is `unknown`, as it already was when the repository did not open. Nothing panics.
- The git2 dependency goes; `build` becomes `["dep:serde", "dep:toml"]`.

Out of scope: the consumers' build.rs files, which call `git_envs` unchanged.
