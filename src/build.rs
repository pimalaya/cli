//! Helpers for binary build scripts.
//!
//! Small functions a build.rs calls to expose enabled features, target
//! platform and git metadata as environment variables baked into the
//! binary.

use std::{
    collections::BTreeMap,
    env::{self, VarError},
    process::Command,
};

use serde::Deserialize;

/// Builds the `CARGO_FEATURES` environment variable.
///
/// This function turns enabled cargo features into a simple string
/// `+feature1 +feature2 +featureN`, which then exposes it via the
/// `CARGO_FEATURES` environment variable.
///
/// It first reads and parses the Cargo.toml in order to extract all
/// available features (omitting "default"). It then checks for
/// enabled features via `CARGO_FEATURE_<name>` to finally collect
/// them into a string.
pub fn features_env(cargo: &str) {
    #[derive(Deserialize)]
    struct Config {
        features: BTreeMap<String, Vec<String>>,
    }

    impl Config {
        fn enabled_features(self) -> impl Iterator<Item = String> {
            self.features
                .into_keys()
                .filter(|feature| feature != "default")
                .filter(|feature| {
                    let feature = feature.replace('-', "_").to_uppercase();
                    env::var(format!("CARGO_FEATURE_{feature}")).is_ok()
                })
        }
    }

    let config: Config = toml::from_str(cargo).expect("should parse Cargo.toml");

    let mut features = String::new();

    for feature in config.enabled_features() {
        if !features.is_empty() {
            features.push(' ');
        }
        features.push_str(&format!("+{feature}"));
    }

    println!("cargo::rustc-env=CARGO_FEATURES={features}");
}

/// Builds environment variables related to the target platform.
///
/// This function basically forwards existing cargo environments
/// related to the target platform.
pub fn target_envs() {
    forward_env("CARGO_CFG_TARGET_OS");
    forward_env("CARGO_CFG_TARGET_ENV");
    forward_env("CARGO_CFG_TARGET_ARCH");
}

/// Builds environment variables related to git.
///
/// This function forwards `GIT_DESCRIBE` and `GIT_REV` when they are
/// set, else builds them with the git command, or `unknown` if it fails.
pub fn git_envs() {
    if try_forward_env("GIT_DESCRIBE").is_err() {
        let description = git(&["describe", "--all", "--always"]);
        println!("cargo::rustc-env=GIT_DESCRIBE={description}");
    }

    if try_forward_env("GIT_REV").is_err() {
        let rev = git(&["rev-parse", "HEAD"]);
        println!("cargo::rustc-env=GIT_REV={rev}");
    }
}

/// Runs git on the working directory's repository, never a parent's.
fn git(args: &[&str]) -> String {
    let output = Command::new("git")
        .arg("--git-dir=.git")
        .args(args)
        .output();

    match output {
        Ok(output) if output.status.success() => {
            String::from_utf8_lossy(&output.stdout).trim().to_owned()
        }
        _ => String::from("unknown"),
    }
}

/// Tries to forward the given environment variable.
///
/// For a more strict version, see [`forward_env`].
fn try_forward_env(key: &str) -> Result<String, VarError> {
    let env = env::var(key);

    if let Ok(val) = &env {
        println!("cargo::rustc-env={key}={val}");
    }

    env
}

/// Forwards the given environment variable.
///
/// This function panics in case the forward fails (when the
/// environment variable does not exist for example).
///
/// For a less strict version, see [`try_forward_env`].
fn forward_env(key: &str) {
    try_forward_env(key).unwrap_or_else(|_| panic!("should get env {key}"));
}
