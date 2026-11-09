---
cairn: tasks
change: build-scripts-run-git
---

# Tasks

- [x] src/build.rs: `git_envs` through the git command with `--git-dir=.git`, output trimmed, `unknown` on any failure; doc comment no longer names git2
- [x] Cargo.toml: git2 removed, `build` takes serde and toml only
- [x] Scratch consumer build script, run on git2 then on the git command: same values in this checkout, `unknown` outside a repository and in a subdirectory of one, forwarded variables kept, `unknown` with git off PATH
- [x] fmt, clippy with all features, with `terminal` alone and `build` alone, tests, cargo deny
- [x] CHANGELOG under [Unreleased]
- [x] Fold delta.md into cairn/spec/build.md (new); log; status `landed`
