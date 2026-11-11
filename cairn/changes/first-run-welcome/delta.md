---
cairn: delta
change: first-run-welcome
---

## ADDED Requirements

### Requirement: A printed command runs as pasted
`shell::quote` SHALL wrap a string in single quotes, each embedded `'` written `'\''`, so a POSIX shell reads it back unchanged; on Windows it SHALL wrap it in double quotes, which cmd and PowerShell both read. `shell::invocation` SHALL write the binary name, then ` -c ` and the quoted path for each configuration path in order, then ` -a ` and the quoted account name when one is given and not empty.

#### Scenario: A quote in a path and an account
- GIVEN the binary `app`, the paths `/a.toml` and `/b c.toml`, and the account `work's`
- WHEN the invocation is built on Unix
- THEN it reads `app -c '/a.toml' -c '/b c.toml' -a 'work'\''s'`

### Requirement: A missing configuration meets a welcome
`welcome::Welcome` SHALL print the binary's intro line; a blank line; `No configuration found at <path>. Create it from the sample configuration:`; a blank line; the commands creating the file, each indented two spaces; a blank line; the binary's next-step sentence; a blank line; and the first command, indented two spaces, starting with the binary's invocation carrying its `-c` paths and no account. With a shipped sample, the commands SHALL be `mkdir -p <dir>` (`mkdir` on Windows) when the file's parent directory is named and missing, then `install -m 644 <sample> <file>` (`copy` on Windows); without one, `curl -sSL --create-dirs -o <file> <url>` (`curl.exe` on Windows). Paths SHALL be quoted with `shell::quote`. Every line SHALL end with a newline. `welcome::is_wanted` SHALL be true when the printer does not print JSON and stderr is a terminal.

#### Scenario: No sample shipped
- GIVEN the missing file `/a b/it's.toml`, given with `-c`, and no shipped sample
- WHEN the welcome prints on Unix
- THEN it holds the line `  curl -sSL --create-dirs -o '/a b/it'\''s.toml' <url>` and ends with `  <bin> -c '/a b/it'\''s.toml' <first command>`

#### Scenario: A shipped sample and a missing directory
- GIVEN the same file, its directory missing, and a shipped sample
- WHEN the welcome prints on Unix
- THEN the commands are `  mkdir -p '/a b'` then `  install -m 644 '<sample>' '/a b/it'\''s.toml'`, and no `curl`

### Requirement: The shipped sample is found next to the binary
`welcome::shipped_sample` SHALL take the running binary's path, canonicalized except on Windows, and SHALL answer the first existing file among share/doc/<bin>/config.sample.toml under the parent of the binary's directory, where a package installs it, and under the binary's directory, where an extracted release archive puts it, or nothing.

#### Scenario: An installed and an extracted sample
- GIVEN a prefix holding bin/ and share/doc/<bin>/config.sample.toml
- WHEN the sample is looked up from bin/ or from the prefix itself
- THEN both find that file, and neither does before it exists

## MODIFIED Requirements

## REMOVED Requirements
