//! Interactive account setup wizards, one submodule per protocol.
//!
//! Each wizard prompts for the settings of one account and returns a
//! `Wizard*Config` the caller maps into its own configuration. The
//! keyring, ortie and review modules hold the steps every wizard shares:
//! the secret, the OAuth hand-off and the final review.

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
#[cfg(feature = "wizard")]
pub mod ortie;
#[cfg(feature = "wizard")]
pub mod review;
#[cfg(feature = "smtp")]
pub mod smtp;
