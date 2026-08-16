```sh
# Identifier types only — no schema, no serde_json, no date/decimal deps.
cargo add rubo4e

# The usual application setup: generated types, JSON, real dates and decimals.
cargo add rubo4e --features versioned,json,time,decimal
```
