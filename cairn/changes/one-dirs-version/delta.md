---
cairn: delta
change: one-dirs-version
---

## ADDED Requirements

### Requirement: Path arguments expand like a shell
`clap::parsers::path_parser` SHALL expand a leading `~` to the home directory dirs reports, and `$NAME` and `${NAME}` to the environment variable `NAME`, then SHALL canonicalize the path, keeping the expanded path when it does not exist. A home directory that is not valid Unicode SHALL leave the `~` as written. A path naming an unset variable SHALL be rejected with `error looking key '<NAME>' up: environment variable not found`.

#### Scenario: The home directory three ways
- GIVEN `HOME` set to `/home/user`, which holds no `x`
- WHEN `~/x`, `$HOME/x` and `${HOME}/x` are parsed
- THEN each gives `/home/user/x`

#### Scenario: An unset variable
- GIVEN no `UNSET` variable in the environment
- WHEN `$UNSET/file` is parsed
- THEN it is rejected with `error looking key 'UNSET' up: environment variable not found`

### Requirement: One dirs version
shellexpand SHALL be built with `base-0` alone, without its `tilde` feature, the home directory coming from pimalaya-cli's own dirs dependency, so a binary carries one dirs version.
