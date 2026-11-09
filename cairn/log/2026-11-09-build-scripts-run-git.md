---
cairn: log
change: build-scripts-run-git
landed: 2026-11-09
---

# Build scripts run git

`build::git_envs` (src/build.rs) runs `git --git-dir=.git describe --all --always` for `GIT_DESCRIBE` and `git --git-dir=.git rev-parse HEAD` for `GIT_REV` through `std::process::Command`, in place of git2, when the environment does not already set them. The output is trimmed; a missing git, a directory without a repository or a failing command gives `unknown`. The git2 dependency is gone: `build` is `["dep:serde", "dep:toml"]`, and a consumer's build script compiles no C (git2 brought libgit2-sys and libz-sys, built from C through cc).

`--git-dir=.git` was not in the request: the owner asked for git run from the build script's working directory, as `Repository::open(".")` did. git2's `open` does not search upward, while plain `git` does, so without the flag a crate source in ~/.cargo/registry under a home directory tracked by git (a dotfiles repository) would bake that repository's description into the binary, where git2 gave `unknown`. A subdirectory of a checkout also stays `unknown`, as with git2. `--git-dir` also overrides a `GIT_DIR` set in the environment, as git hooks do; git2's `open` ignored it too.

The panics are gone: git2's `describe` and `head` `expect`s fired in a repository that opened but could not be described, such as one whose HEAD has no commit yet; there `git describe` and `git rev-parse HEAD` exit non-zero and the values are `unknown`. One remaining difference: the abbreviated id in a `GIT_DESCRIBE` suffix (`heads/master-3-g<id>`, or the bare id of a detached HEAD without refs) follows git's `core.abbrev`, auto and at least 7, where libgit2 took 7 lengthened until unique; both give 7 on a repository of this size.

No unit test: what `git_envs` prints depends on the working directory and PATH, process-wide state a unit test cannot change safely. Verified with a scratch consumer whose build script calls `git_envs` (optionally after changing its working directory), built against HEAD 65d6d30 (git2) then this tree: in this checkout both gave `heads/master` and `65d6d3054978fc7fa6895069c0af2fbcaa160c3c`, matching `git describe --all --always` and `git rev-parse HEAD`; outside a repository and in src/ both gave `unknown`; set variables were forwarded; with git off PATH the new code gave `unknown`.

Crates: `cargo tree -e normal,build --no-default-features --features build` went from 25 to 14, losing bitflags, cc, find-msvc-tools, git2, jobserver, libc, libgit2-sys, libz-sys, pkg-config, shlex and vcpkg; Cargo.lock also dropped getrandom 0.4.3 and r-efi 6.0.0. fmt, clippy `-D warnings` with all features and all targets, with `terminal` alone and `build` alone, the 45 unit tests and the doc test pass; cargo deny passes with the same warnings as before.

Spec: build (new capability, 2 requirements added: "Git metadata never fails a build", "Build helpers compile no C").
