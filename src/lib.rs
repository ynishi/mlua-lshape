//! mlua-lshape — mlua wrapper for `lshape` (Pure Lua Schema-as-Data
//! validator + LuaCATS codegen).
//!
//! The five `lshape/*.lua` sources are vendored under `lua/lshape/` and
//! embedded into the binary via [`include_str!`] at compile time, so
//! downstream crates get the full module tree without any runtime
//! filesystem dependency.
//!
//! Typical use from Rust:
//!
//! ```no_run
//! use mlua::Lua;
//! use mlua_lshape::install;
//!
//! let lua = Lua::new();
//! install(&lua).unwrap();
//! let chunk = r#"
//!     local lshape = require("lshape")
//!     local T = lshape.t
//!     local Voted = T.shape({ answer = T.string })
//!     local ok, why = lshape.check.check({ answer = "42" }, Voted)
//!     assert(ok, why)
//! "#;
//! lua.load(chunk).exec().unwrap();
//! ```

use mlua::{Lua, Result, Table, Value};

/// Embedded Lua source files. Key is the `require` path (e.g. `"lshape"`
/// → `lua/lshape/init.lua`), value is the file contents.
///
/// Kept `pub` so downstream crates can embed the same sources (e.g. to
/// inject them into a non-mlua Lua VM or to hash them for drift checks).
pub const LSHAPE_SOURCES: &[(&str, &str)] = &[
    ("lshape", include_str!("../lua/lshape/init.lua")),
    ("lshape.t", include_str!("../lua/lshape/t.lua")),
    ("lshape.check", include_str!("../lua/lshape/check.lua")),
    ("lshape.reflect", include_str!("../lua/lshape/reflect.lua")),
    ("lshape.luacats", include_str!("../lua/lshape/luacats.lua")),
];

/// Install the `lshape.*` modules into the given Lua state's
/// `package.preload` table. After this returns, Lua code can
/// `require("lshape")` / `require("lshape.t")` / etc. normally.
///
/// This does **not** call `require` itself; modules stay lazy until the
/// host Lua code pulls them in.
pub fn install(lua: &Lua) -> Result<()> {
    let package: Table = lua.globals().get("package")?;
    let preload: Table = package.get("preload")?;

    for (name, src) in LSHAPE_SOURCES {
        let name_owned = (*name).to_owned();
        let src_owned = (*src).to_owned();
        let loader = lua.create_function(move |lua, ()| -> Result<Value> {
            let chunk = lua.load(&src_owned).set_name(format!("@{}", name_owned));
            chunk.eval::<Value>()
        })?;
        preload.set(*name, loader)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn install_and_require_lshape() {
        let lua = Lua::new();
        install(&lua).unwrap();
        lua.load(
            r#"
            local lshape = require("lshape")
            assert(type(lshape) == "table")
            assert(type(lshape.t) == "table")
            assert(type(lshape.check) == "table")
            assert(type(lshape.reflect) == "table")
            assert(type(lshape.luacats) == "table")
            "#,
        )
        .exec()
        .unwrap();
    }

    #[test]
    fn check_validates_shape() {
        let lua = Lua::new();
        install(&lua).unwrap();
        lua.load(
            r#"
            local lshape = require("lshape")
            local T = lshape.t
            local Voted = T.shape({ answer = T.string })
            local ok, _ = lshape.check.check({ answer = "42" }, Voted)
            assert(ok)
            local ok2, why = lshape.check.check({ answer = 42 }, Voted)
            assert(not ok2)
            assert(why:find("shape violation"))
            "#,
        )
        .exec()
        .unwrap();
    }

    #[test]
    fn vendored_version_matches() {
        let lua = Lua::new();
        install(&lua).unwrap();
        lua.load(
            r#"
            local lshape = require("lshape")
            assert(lshape._VERSION == "0.3.0",
                "expected lshape._VERSION == '0.3.0', got " .. tostring(lshape._VERSION))

            local T = lshape.t
            -- v0.2.0 surface smoke: any_of, pattern, partial, literal, fn
            local U = T.any_of({ T.string, T.number })
            assert(lshape.check.check("x", U))
            assert(lshape.check.check(1, U))
            assert(not (lshape.check.check(true, U)))

            local Slug = T.pattern("^[a-z]+$")
            assert(lshape.check.check("abc", Slug))
            assert(not (lshape.check.check("ABC", Slug)))

            local P = T.partial({ a = T.string, b = T.number })
            assert(lshape.check.check({}, P))
            assert(lshape.check.check({ a = "x" }, P))

            local L = T.literal("yes")
            assert(lshape.check.check("yes", L))
            assert(not (lshape.check.check("no", L)))

            -- v0.2.0 added: T.fn primitive (function type)
            assert(lshape.check.check(function() end, T.fn))
            assert(not (lshape.check.check(123, T.fn)))

            -- v0.3.0 surface smoke: integer, tuple, bounds
            assert(lshape.check.check(3, T.integer))
            assert(not (lshape.check.check(3.5, T.integer)))

            local Pair = T.tuple({ T.string, T.number })
            assert(lshape.check.check({ "a", 1 }, Pair))
            assert(not (lshape.check.check({ "a" }, Pair)))
            assert(not (lshape.check.check({ "a", 1, true }, Pair)))

            local Age = T.number:min(0):max(150)
            assert(lshape.check.check(30, Age))
            assert(not (lshape.check.check(-1, Age)))
            assert(not (lshape.check.check(151, Age)))

            local Name = T.string:min_len(1):max_len(4)
            assert(lshape.check.check("ab", Name))
            assert(not (lshape.check.check("", Name)))
            assert(not (lshape.check.check("abcde", Name)))
            "#,
        )
        .exec()
        .unwrap();
    }
}
