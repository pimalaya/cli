---
cairn: tasks
change: a-request-off-its-api-is-coded
---

# Tasks

- [x] src/error.rs: `CommonCode::RequestOrigin`, `#[non_exhaustive]`, `request_origin`, `failure_status` exits 1 for it
- [x] Tests: the code reaches the report under context, exits 1, is left alone by `with_code`, and the schema admits it
- [x] Fold into cairn/spec/error.md; CHANGELOG; log
- [ ] Apps, after the next pimalaya-cli release: himalaya, cardamum and calendula raise it through `error::request_origin` (list in the log)
