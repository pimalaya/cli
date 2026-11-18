//! Files only their owner reads.
//!
//! A log trace, a cache or a draft can hold private data: a protocol
//! exchange, the headers of messages, a card. [`private_options`] opens
//! such a file so that, on Unix, it is created readable by its owner
//! alone, whatever the umask, and is never reached through a symbolic
//! link another user planted at its path.

use std::fs::OpenOptions;

/// Options opening a private file, to which the caller adds its mode of
/// access, as in `private_options().create(true).append(true)`.
///
/// On Unix, a file they create gets the permissions 0600, and opening a
/// symbolic link fails (`O_NOFOLLOW`) instead of following it; a file
/// that exists keeps its permissions. Elsewhere they are plain options.
pub fn private_options() -> OpenOptions {
    #[allow(unused_mut)]
    let mut options = OpenOptions::new();

    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;

        options.mode(0o600).custom_flags(libc::O_NOFOLLOW);
    }

    options
}
