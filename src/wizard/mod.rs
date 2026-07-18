//! Interactive account setup wizards, one submodule per protocol.
//!
//! Each wizard prompts for the settings of one account and returns a
//! `Wizard*Config` the caller maps into its own configuration.

#[cfg(feature = "caldav")]
pub mod caldav;
#[cfg(feature = "carddav")]
pub mod carddav;
#[cfg(feature = "imap")]
pub mod imap;
#[cfg(feature = "jmap")]
pub mod jmap;
#[cfg(feature = "wizard")]
pub mod keyring;
#[cfg(feature = "smtp")]
pub mod smtp;
