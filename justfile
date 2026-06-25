# mlua-lshape — task runner
#
# Usage:
#   just                  # list tasks
#   just test             # cargo test (mlua wrapper + Pure Lua vendored)
#   just vendor           # mlua-pkg install (sync lua/lshape from upstream)
#   just update-deps      # mlua-pkg update (bump SemVer-prefix tag pins)
#   just check-vendor     # assert lua/lshape is in sync with mlua-pkg.lock
#   just release-prep     # local smoke before bump (fmt + clippy + test + vendor diff)
#
# Prereqs:
#   - cargo (Rust 1.77+, per Cargo.toml rust-version)
#   - mlua-pkg on PATH (cargo install mlua-pkg)
#   - git (for check-vendor diff)

MLUA_PKG := "mlua-pkg"

default:
    @just --list

# Rust ------------------------------------------------------------------

# [group('allow-agent')]
fmt:
    cargo fmt --all

# [group('allow-agent')]
fmt-check:
    cargo fmt --all -- --check

# [group('allow-agent')]
clippy:
    cargo clippy --all-targets -- -D warnings

# [group('allow-agent')]
test:
    cargo test

# mlua-pkg consumer -----------------------------------------------------

# Re-vendor lua/lshape from upstream per mlua-pkg.toml.
# [group('allow-agent')]
vendor:
    {{MLUA_PKG}} install

# Refresh deps and bump SemVer-prefix tag pins (e.g. tag="v0.2" → highest v0.2.x).
# [group('allow-agent')]
update-deps:
    {{MLUA_PKG}} update

# Assert lua/lshape matches mlua-pkg.lock — fails if vendor needs refresh.
# [group('allow-agent')]
check-vendor:
    #!/usr/bin/env bash
    set -euo pipefail
    {{MLUA_PKG}} install
    if ! git diff --quiet -- lua/ mlua-pkg.lock; then
        echo "ERROR: vendored Lua out of sync with mlua-pkg.lock" >&2
        git diff --stat -- lua/ mlua-pkg.lock >&2
        exit 1
    fi
    echo "vendor in sync"

# Release ---------------------------------------------------------------

# Local smoke before bump. Runs fmt-check + clippy + test + vendor sync check.
# [group('allow-agent')]
release-prep:
    just fmt-check
    just clippy
    just test
    just check-vendor
