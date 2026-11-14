//! # Capability
//!
//! The output of every binary's `account capability` command: one row per
//! capability of the unified API, the backend serving it and how well. It
//! prints as a [`Listing`] headed by the account.
//!
//! [`Listing`]: crate::listing::Listing

use std::fmt;

use schemars::JsonSchema;
use serde::Serialize;

use crate::{listing::Listing, sanitize};

/// The `account capability` output.
#[derive(Clone, Debug, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct AccountCapabilityOutput {
    /// The account the capabilities are resolved for.
    pub account: String,
    /// One row per unified capability, in the order the commands reach
    /// them.
    pub capabilities: Vec<AccountCapability>,
}

/// How the account serves one unified capability.
#[derive(Clone, Debug, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct AccountCapability {
    /// The capability, `<noun>.<verb>` and its refinements
    /// (`message.move`, `event.search.body`).
    pub name: String,
    /// The backend it resolves to, `null` when the account configures none
    /// that `--backend` allows.
    pub backend: Option<String>,
    /// How well the backend serves it.
    pub level: CapabilityLevel,
    /// What the level leaves out, `null` for a full one.
    pub detail: Option<String>,
}

/// How well a backend serves a capability.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum CapabilityLevel {
    /// Served whole.
    Full,
    /// Served with the limits the detail names.
    Partial,
    /// Decided by the server once connected.
    Server,
    /// Not served: the command is refused before connecting.
    None,
}

impl fmt::Display for CapabilityLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Full => "full",
            Self::Partial => "partial",
            Self::Server => "server",
            Self::None => "none",
        })
    }
}

impl fmt::Display for AccountCapabilityOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut listing = Listing::new(&self.account, None);

        for capability in &self.capabilities {
            let detail = capability.detail.as_deref().unwrap_or_default();

            listing.items.push([
                sanitize::text(&capability.name).into_owned(),
                capability.backend.clone().unwrap_or_default(),
                capability.level.to_string(),
                sanitize::text(detail).into_owned(),
            ]);
        }

        write!(f, "{listing}")
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::capability::*;

    #[test]
    fn capabilities_print_one_line_each_under_the_account() {
        let output = AccountCapabilityOutput {
            account: String::from("work"),
            capabilities: vec![
                AccountCapability {
                    name: String::from("message.move"),
                    backend: Some(String::from("imap")),
                    level: CapabilityLevel::Full,
                    detail: None,
                },
                AccountCapability {
                    name: String::from("message.send"),
                    backend: None,
                    level: CapabilityLevel::None,
                    detail: Some(String::from("No transport")),
                },
            ],
        };

        let expected = "\
Account:    work

message.move  imap  full  -
message.send  -     none  No transport
";
        assert_eq!(output.to_string(), expected);

        let json = serde_json::to_value(&output).unwrap();
        let second = json!({
            "name": "message.send",
            "backend": null,
            "level": "none",
            "detail": "No transport",
        });
        assert_eq!(json["capabilities"][1], second);
    }
}
