# dprint-plugin-sql

[![](https://img.shields.io/crates/v/dprint-plugin-sql.svg)](https://crates.io/crates/dprint-plugin-sql) [![CI](https://github.com/dprint/dprint-plugin-sql/workflows/CI/badge.svg)](https://github.com/dprint/dprint-plugin-sql/actions?query=workflow%3ACI)

Wrapper around [sqlformat-rs](https://github.com/shssoichiro/sqlformat-rs) for use as a formatting plugin for [dprint](https://github.com/dprint/dprint).

## Configuration

### `uppercase: bool`

When set, changes reserved keywords to ALL CAPS
**Default:** `false`

### `lines_between_queries: number`

Controls the number of line breaks after a query
**Default:** `1`

### `ignore_case_convert: string[]`

Ignore case conversion for specified strings in the array
**Default:** `[]`

### `inline: bool`

Keep the query in a single line
**Default:** `false`

### `max_inline_block: number`

Maximum length of an inline block
**Default:** `50`

### `max_inline_arguments: number`

Maximum length of inline arguments
If unset, keep every argument in a separate line
**Default:** `Unlimited`

### `max_inline_top_level: number`

Inline the argument at the top level if they would fit a line of this length
**Default:** `Unlimited`

### `joins_as_top_level: bool`

Consider any JOIN statement as a top level keyword instead of a reserved keyword
**Default:** `false`
