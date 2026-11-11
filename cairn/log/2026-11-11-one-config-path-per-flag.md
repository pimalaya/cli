---
cairn: log
change: one-config-path-per-flag
landed: 2026-11-11
---

# One path per `-c`

`clap::args::ConfigFlags` lost `value_delimiter = ':'`: each `-c` holds one path, the flag repeated to merge files. Its help is Ortie's `ConfigPathsArg` doc, with no wizard in it; the type and its `paths` field keep their names and clap ids (`config_paths`), so a binary swaps its own flag for it. `a_config_path_is_never_split`, ported from Ortie, parses `-c 'C:\a.toml' -c 'b:c;d.toml'` and expects the two paths whole; with the delimiter put back it failed with `["C", "\\a.toml", "b", "c;d.toml"]`.

No binary uses `ConfigFlags` today: himalaya, cardamum, calendula, neverest, carillon, sirup, himalaya-tui, comodoro, journal and alarmd each declare their own `-c` splitting on a delimiter (himalaya also reads `HIMALAYA_CONFIG`).

Spec: terminal ("One configuration path per flag" added).
