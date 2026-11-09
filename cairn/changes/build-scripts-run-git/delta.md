---
cairn: delta
change: build-scripts-run-git
---

## ADDED Requirements

### Requirement: Git metadata never fails a build
`build::git_envs` SHALL forward `GIT_DESCRIBE` and `GIT_REV` when the environment sets them. Otherwise it SHALL set `GIT_DESCRIBE` to the output of `git describe --all --always` and `GIT_REV` to that of `git rev-parse HEAD`, trimmed, both run on the repository whose .git sits in the build script's working directory, never on a parent directory's. When git is missing, the directory holds no repository or the command fails, the variable SHALL be `unknown`, and the build script SHALL NOT panic.

#### Scenario: A build in a checkout
- GIVEN a checkout of the branch `master`, neither variable set
- WHEN the build script calls `git_envs`
- THEN `GIT_DESCRIBE` is `heads/master` and `GIT_REV` is HEAD's full commit id

#### Scenario: A build without a repository
- GIVEN a source tree without .git, as in a Nix sandbox, neither variable set
- WHEN the build script calls `git_envs`
- THEN `GIT_DESCRIBE` and `GIT_REV` are `unknown`

#### Scenario: A crate inside another repository
- GIVEN a crate source without .git, under a directory tracked by another repository
- WHEN the build script calls `git_envs`
- THEN `GIT_DESCRIBE` and `GIT_REV` are `unknown`

#### Scenario: Variables set by the packager
- GIVEN `GIT_DESCRIBE=v1.0.0` and `GIT_REV=abc` in the environment
- WHEN the build script calls `git_envs`
- THEN both are forwarded as set, and git does not run

### Requirement: Build helpers compile no C
The `build` feature SHALL depend on no crate that compiles C code or links a system library, so a binary's build script needs no C compiler: git metadata comes from the git command.
