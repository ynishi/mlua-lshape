# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-04-21

Initial public release. mlua wrapper that installs the
[`lshape`](https://github.com/ynishi/lshape) module tree into a Lua VM
via `package.preload`. The Pure Lua sources are vendored under
`lua/lshape/` and embedded at compile time with `include_str!`, so
downstream builds stay hermetic (no LuaRocks, no filesystem probing).

### Added

- `install(&Lua) -> Result<()>` — registers `lshape`, `lshape.t`,
  `lshape.check`, `lshape.reflect`, `lshape.luacats` into
  `package.preload`; modules stay lazy until the host Lua code calls
  `require`.
- `LSHAPE_SOURCES: &[(&str, &str)]` — public embedded-source table for
  downstream consumers that want to inject the same sources into
  another Lua VM or hash them for drift checks.
- Vendored [lshape v0.1.0](https://github.com/ynishi/lshape/releases/tag/v0.1.0):
  `T.shape` / `T.partial` / `T.array_of` / `T.map_of` / `T.one_of` /
  `T.literal` / `T.any_of` / `T.discriminated` / `T.pattern` /
  `T.ref`, `check` / `assert` / `assert_dev`, `reflect`, LuaCATS
  codegen.
- Unit tests: install + require round-trip, basic shape validation,
  and v0.1.0 surface smoke (`_VERSION` pin + `any_of` / `pattern` /
  `partial` / `literal`) so the embedded copy cannot silently drift
  from upstream.
- Dual MIT / Apache-2.0 licensing.

[0.1.0]: https://github.com/ynishi/mlua-lshape/releases/tag/v0.1.0
