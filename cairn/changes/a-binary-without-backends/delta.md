---
cairn: delta
change: a-binary-without-backends
---

## MODIFIED Requirements

### Requirement: Every binary carries an account group
Appended sentence: A binary opening no backend, such as Ortie, SHALL carry the group all the same, `account list` reaching nothing and `account check` opening what an account stands on (its token storage and its endpoints for Ortie), and MAY add `account show`, printing every property of one account, reaching nothing.

### Requirement: Shared nouns keep one shape
Appended sentence: The account row, the checked account and the backend check row bind the binaries opening backends; a binary opening none SHALL print its own account shapes, named after what it lists and checks, its check rows keeping `ok`, `code`, `error` and `tls` with the meaning given here.

### Requirement: The accounts and their checks print alike
First sentences reworded: `account::AccountListOutput` SHALL be the output of every binary's `account list` and `account::AccountCheckOutput` the output of every binary's `account check`, for the binaries opening backends; a binary opening none prints its own shapes (cairn/spec/output.md, "Shared nouns keep one shape").
