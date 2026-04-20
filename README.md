# mlua-lshape

[mlua](https://crates.io/crates/mlua) wrapper for
[lshape](https://github.com/ynishi/lshape) — Pure Lua Schema-as-Data
validator + LuaCATS codegen. The `lshape/*.lua` sources are vendored
into this crate (under `lua/lshape/`) and embedded via `include_str!`
at compile time.

## Quick start

```rust
use mlua::Lua;
use mlua_lshape::install;

let lua = Lua::new();
install(&lua)?;
lua.load(r#"
    local lshape = require("lshape")
    local T = lshape.t
    local Voted = T.shape({ answer = T.string, reasoning = T.string })
    local ok, why = lshape.check.check({ answer = "42", reasoning = "..." }, Voted)
    assert(ok, why)
"#).exec()?;
```

## Why vendored?

The upstream `lshape` is Pure Lua with no package manager dependency.
Shipping the sources inside this crate keeps downstream builds
hermetic — no LuaRocks, no filesystem probing, no version drift. The
same pattern is used by
[`mlua-probe-mcp`](https://crates.io/crates/mlua-probe-mcp) /
[`mlua-lspec`](https://crates.io/crates/mlua-lspec).

## Vendored sources

`lua/lshape/*.lua` is a verbatim copy of the upstream
[ynishi/lshape](https://github.com/ynishi/lshape) repository. Refresh
via:

```bash
cp /path/to/lshape/lshape/*.lua lua/lshape/
```

The `LSHAPE_SOURCES` public constant exposes the embedded sources
(`&[(name, src)]`) for downstream consumers that want to inject them
into another Lua VM or hash them for drift checks.

## License

Dual-licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](./LICENSE-APACHE) or
  <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](./LICENSE-MIT) or
  <http://opensource.org/licenses/MIT>)

at your option. The vendored `lua/lshape/` files are dual-licensed under
the same terms, copyright Yutaka Nishimura.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual-licensed as above, without any additional terms
or conditions.
