---
cairn: delta
change: one-config-path-per-flag
---

## ADDED Requirements

### Requirement: One configuration path per flag
`clap::args::ConfigFlags` SHALL take one path per `-c` or `--config`, given as many times as there are files, the first one the base and the next ones merged on top. A value SHALL NOT be split, whatever it holds, and no environment variable SHALL supply it. Each path SHALL go through `path_parser`.

#### Scenario: A Windows path and a colon
- GIVEN `-c 'C:\a.toml' -c 'b:c;d.toml'`
- WHEN the arguments are parsed
- THEN `paths` holds exactly `C:\a.toml` then `b:c;d.toml`

## MODIFIED Requirements

## REMOVED Requirements
