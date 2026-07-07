# 0002. Use sqlx exclusively, not rusqlite

Status: Accepted

## Context

Early on, crustly's database layer considered both `sqlx` (async, compile-time
checked queries) and `rusqlite` (a lower-level synchronous SQLite binding),
with `refinery` for migrations on top of either. In practice:

- `rusqlite 0.31` depends on `libsqlite3-sys 0.28`.
- `sqlx 0.7` depends on `libsqlite3-sys 0.26`.
- Cargo cannot link two different versions of the same native library
  (`libsqlite3-sys`) into one binary, so pulling in both crates broke the
  build.

(See `docs/guides/BUILD_NOTES.md` § "SQLite Conflict Resolution" for the
original write-up.)

## Decision

We use `sqlx` exclusively for all database access, and `sqlx-cli` for
migrations instead of `refinery`. `rusqlite` is not a dependency.

## Consequences

One database crate, one migration tool, no native-library version conflict.
The cost: `sqlx`'s compile-time query checking needs a `DATABASE_URL` /
`.sqlx` offline cache available at build time, and any future crate that
transitively pulls in `rusqlite` (directly or via `libsqlite3-sys`) will
reintroduce this exact conflict - check `cargo tree -i libsqlite3-sys` before
adding a new SQLite-adjacent dependency.
