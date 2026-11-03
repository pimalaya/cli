//! Output shapes shared by every binary.
//!
//! [`Bytes`] carries binary content under JSON, never written raw to
//! stdout nor decoded lossily. [`Paginated`] carries a listing beside
//! the cursor of its next page, which the `--page-token` argument of
//! [`PageTokenArg`] takes back.
//!
//! [`PageTokenArg`]: crate::clap::args::PageTokenArg

use std::{borrow::Cow, fmt};

use base64_codec::codec::Base64Codec;
use schemars::{JsonSchema, Schema, SchemaGenerator, json_schema};
use serde::{Serialize, Serializer, ser::SerializeStruct};

/// Binary content, printed as `{data, size, mime}` with `data` in
/// base64, encoded on the CPU token the binary installed (simd-cpu).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Bytes {
    /// The content.
    pub data: Vec<u8>,
    /// The media type of the content, `application/octet-stream` when
    /// unknown.
    pub mime: String,
}

impl fmt::Display for Bytes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{} bytes of {}", self.data.len(), self.mime)
    }
}

impl Serialize for Bytes {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut data = Vec::new();
        Base64Codec::default().encode_vec(&self.data, &mut data);
        // NOTE: the base64 alphabet is ASCII, so this never falls back.
        let data = String::from_utf8(data).unwrap_or_default();

        let mut bytes = serializer.serialize_struct("Bytes", 3)?;
        bytes.serialize_field("data", &data)?;
        bytes.serialize_field("size", &self.data.len())?;
        bytes.serialize_field("mime", &self.mime)?;
        bytes.end()
    }
}

impl JsonSchema for Bytes {
    fn schema_name() -> Cow<'static, str> {
        "Bytes".into()
    }

    fn json_schema(_: &mut SchemaGenerator) -> Schema {
        json_schema!({
            "description": "Binary content.",
            "type": "object",
            "properties": {
                "data": {
                    "description": "The content, in base64.",
                    "type": "string",
                    "contentEncoding": "base64",
                },
                "size": {
                    "description": "The size of the content, in bytes.",
                    "type": "integer",
                    "format": "uint64",
                    "minimum": 0,
                },
                "mime": {
                    "description": "The media type of the content.",
                    "type": "string",
                },
            },
            "required": ["data", "size", "mime"],
        })
    }
}

/// A listing plus the cursor of its next page.
///
/// The cursor is part of the output, a footer line in text and a
/// `nextPage` field in JSON, `null` once the listing is complete, so a
/// script reads it and gives it back through `--page-token`.
#[derive(Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Paginated<T> {
    /// The listing.
    #[serde(flatten)]
    pub inner: T,
    /// The cursor of the next page, `null` once the listing is complete.
    pub next_page: Option<String>,
}

impl<T: fmt::Display> fmt::Display for Paginated<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner)?;

        if let Some(cursor) = &self.next_page {
            writeln!(f, "Next page: {cursor}")?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::{clap::commands::output_schema, output::*};

    #[derive(Serialize, JsonSchema)]
    struct Rows {
        rows: Vec<u8>,
    }

    #[test]
    fn bytes_print_as_base64_with_their_size() {
        let bytes = Bytes {
            data: b"\xff\x00a".to_vec(),
            mime: "application/octet-stream".into(),
        };

        assert_eq!(
            serde_json::to_value(bytes).unwrap(),
            json!({ "data": "/wBh", "size": 3, "mime": "application/octet-stream" })
        );
    }

    #[test]
    fn a_complete_listing_prints_a_null_cursor() {
        let page = Paginated {
            inner: Rows { rows: vec![1] },
            next_page: None,
        };

        assert_eq!(
            serde_json::to_value(page).unwrap(),
            json!({ "rows": [1], "nextPage": null })
        );
    }

    #[test]
    fn the_cursor_is_required_in_the_schema() {
        let schema = output_schema::<Paginated<Rows>>();
        let required = schema["required"].as_array().unwrap();

        assert!(required.contains(&json!("nextPage")), "{schema}");
        assert!(required.contains(&json!("rows")), "{schema}");
    }
}
