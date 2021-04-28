[![Crate release version](https://flat.badgen.net/crates/v/clearscreen)](https://crates.io/crates/clearscreen)
[![Crate license: Apache 2.0 or MIT](https://flat.badgen.net/badge/license/Apache%202.0%20or%20MIT)][copyright]
![MSRV: 1.51.0 (breaking)](https://flat.badgen.net/badge/MSRV/1.51.0%20%28breaking%29/green)
[![CI status](https://github.com/watchexec/clearscreen/actions/workflows/check.yml/badge.svg)](https://github.com/watchexec/clearscreen/actions/workflows/check.yml)
[![Uses Caretaker Maintainership](https://flat.badgen.net/badge/Caretaker/Maintainership%20👥%20/purple)][caretaker]

# ClearScreen

_Cross-platform terminal screen clearing library._

- **[API documentation][docs]**.
- [Dual-licensed][copyright] with Apache 2.0 and MIT.
- Uses [Caretaker Maintainership][caretaker].
- Minimum Supported Rust Version: 1.51.0.

[caretaker]: ./CARETAKERS.md
[copyright]: ./COPYRIGHT
[docs]: https://docs.rs/clearscreen

Tested with and tweaked for over 50 different terminals, multiplexers, SSH clients.
See my research notes in the [TERMINALS.md](./TERMINALS.md) file.

## Quick start

```toml
[dependencies]
clearscreen = "1.0.0"
```

```rust
clearscreen::clear().unwrap();
```
