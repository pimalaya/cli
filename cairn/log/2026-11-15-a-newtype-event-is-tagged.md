---
cairn: log
change: a-newtype-event-is-tagged
landed: 2026-11-15
---

# A newtype event is tagged

Found by pimalaya-70 adopting the streaming shape in comodoro: an events enum with a newtype variant, `TimerChanged(TimerOutput)` under `#[serde(tag = "event")]`, which schemars writes as `{"$ref": ..., "properties": {"event": ...}, "required": ["event"]}` (valid 2020-12), failed the registry check as "event without an event key": `collect_alternatives` followed the `$ref` and read the payload, which has no tag.

## What landed

- `collect_alternatives` takes a `$ref` carrying `properties` or `required` of its own as one alternative, read by those keywords.
- `a_newtype_event_variant_is_tagged`, on a real schemars schema; it fails with the old reading.
- The spec also says the text form of a result printed on a signal says the command stopped.

## Capabilities moved

- [output](../spec/output.md): *Events are described apart* and *A long-lived command streams events* amended.
