# Patchmixolint

A personal set of lints for the [Dylint](https://github.com/trailofbits/dylint)
utility. These are incredibly opinionated and will probably produce warnings
that you don't care about (ex. linting on a missing lint level declaration for
`meta_variable_misuse` in a crate that will never use macros).

The code for the lints themselves might not be that great. Please tread with
caution!

**Currently supports** `rustc 1.59.0-nightly (0b6f079e4 2021-12-07)`.

## Setting Patchmixolint lint levels
You can modify Patchmixolint's lint levels in about the way you'd expect.

* You can specify them in `DYLINT_RUSTFLAGS`:
  ```
  $ DYLINT_RUSTFLAGS="-Apatchmixolint::missing_lints" cargo dylint $TARGET_DIR/libpatchmixolint@nightly-m68k-unknown-genesis.so
  ```
* You can declare them in `main.rs`/`lib.rs`. Since Patchmixolint registers its
  lints as tool lints, you must use `#![feature(register_tool)]` for this to
  work. This shouldn't affect normal operation.
```rust
#![allow(patchmixolint::macro_rules_over_macro)] // << lint level changed
#![feature(decl_macro, register_tool)] // << register_tool feature must be enabled
#![register_tool(patchmixolint)] // << register `patchmixolint`

macro_rules! dont_change_me {
  () => {};
}
```

