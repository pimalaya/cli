---
cairn: log
change: the-item-error-key-names-no-command
landed: 2026-11-14
---

# The item error key names no command

src/clap/commands/json_schema.rs: `JsonSchemaRegistry::check` no longer reports `<bin>-item-error` as a schema naming no command, as it already spared `<bin>-error`. Himalaya registers `himalaya-item-error` for the entries of its `errors` lists (a-result-carries-its-errors) and had to remove the key before checking. The check's test registers both keys.

Spec: output ("Every command that prints SHALL register its schema", the item error's key spared).
