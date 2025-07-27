# dprint-plugin-sql

[![](https://img.shields.io/crates/v/dprint-plugin-sql.svg)](https://crates.io/crates/dprint-plugin-sql) [![CI](https://github.com/dprint/dprint-plugin-sql/workflows/CI/badge.svg)](https://github.com/dprint/dprint-plugin-sql/actions?query=workflow%3ACI)

Wrapper around [sqlformat-rs](https://github.com/shssoichiro/sqlformat-rs) for use as a formatting plugin for [dprint](https://github.com/dprint/dprint).

## Configuration

### `Uppercase: bool`

When set, changes reserved keywords to ALL CAPS
**Default:** `false`

### `LinesBetweenQueries: number`

Controls the number of line breaks after a query
**Default:** `1`

### `IgnoreCaseConvert: string[]`

Ignore case conversion for specified strings in the array
**Default:** `[]`

### `Inline: bool`

Keep the query in a single line
**Default:** `false`

### `MaxInlineBlock: number`

Maximum length of an inline block
**Default:** `50`

### `MaxInlineArguments: number`

Maximum length of inline arguments
If unset, keep every argument in a separate line
**Default:** `Unlimited`

### `MaxInlineTopLevel: number`

Inline the argument at the top level if they would fit a line of this length
**Default:** `Unlimited`

### `JoinsAsTopLevel: bool`

Consider any JOIN statement as a top level keyword instead of a reserved keyword
**Default:** `false`
