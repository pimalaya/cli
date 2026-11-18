---
cairn: delta
change: the-log-file-is-private
---

## ADDED Requirements

### Requirement: A private file is its owner's
`file::private_options` SHALL answer `OpenOptions` which, on Unix, create a file with the permissions 0600 whatever the umask and fail on a symbolic link at the path rather than follow it; a file that exists SHALL keep its permissions. Elsewhere they SHALL be plain options. The caller SHALL add its mode of access.

### Requirement: The log file is private
`Logger::try_init` SHALL open the `--log-file` file in append mode through `file::private_options`, so a log file it creates on Unix is readable by its owner alone and a symbolic link at its path is refused. A log file it cannot open SHALL fail the command with `Cannot open log file <path>`, the OS error as its source.

#### Scenario: A link planted at the log path
- GIVEN a symbolic link `trace.log` pointing to a file that does not exist
- WHEN a binary runs with `--log-file trace.log`
- THEN it fails with an error naming `trace.log`, and the link's target is not created

#### Scenario: A new log file
- GIVEN a umask of 022 and no file at `trace.log`
- WHEN a binary runs with `--log-file trace.log`
- THEN `trace.log` is created with the permissions 0600

## MODIFIED Requirements

## REMOVED Requirements
