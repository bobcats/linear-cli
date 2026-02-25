# Release Workflow: Prebuilt Binaries via GitHub Releases

**Date:** 2026-02-25
**Status:** Accepted

## Problem

The only installation path is `cargo install --path .` — users must have the Rust toolchain. This is the biggest adoption barrier.

## Decision

Tag-push driven GitHub Releases with prebuilt binaries for five targets. No crates.io publish, no Homebrew tap, no install scripts — those can layer on later.

## Trigger

Push a version tag:

```bash
git tag v0.1.0
git push --tags
```

The workflow `.github/workflows/release.yml` triggers on `tags: ["v*"]`.

## Targets

| Target | Runner | Build Method |
|--------|--------|-------------|
| `x86_64-unknown-linux-gnu` | `ubuntu-latest` | `cargo build --release` |
| `aarch64-unknown-linux-gnu` | `ubuntu-latest` | `cross build --release` |
| `x86_64-apple-darwin` | `macos-latest` | `cargo build --release --target x86_64-apple-darwin` |
| `aarch64-apple-darwin` | `macos-latest` | `cargo build --release` |
| `x86_64-pc-windows-msvc` | `windows-latest` | `cargo build --release` |

macOS runners are Apple Silicon (aarch64) natively. The x86_64-macos build uses `rustup target add` and an explicit `--target` flag.

aarch64-linux uses `cross` (Docker-based cross-compilation) since there's no native ARM runner needed.

## Artifact Naming

```
linear-cli-v0.1.0-x86_64-linux.tar.gz
linear-cli-v0.1.0-aarch64-linux.tar.gz
linear-cli-v0.1.0-x86_64-macos.tar.gz
linear-cli-v0.1.0-aarch64-macos.tar.gz
linear-cli-v0.1.0-x86_64-windows.zip
```

Tarballs for Unix, zip for Windows. Each archive contains the binary.

## Workflow Shape

```
tag push
  └─► release.yml triggers
        └─► build (5 parallel jobs)
        │     ├─ x86_64-linux    → .tar.gz
        │     ├─ aarch64-linux   → .tar.gz
        │     ├─ x86_64-macos    → .tar.gz
        │     ├─ aarch64-macos   → .tar.gz
        │     └─ x86_64-windows  → .zip
        │
        └─► release (after all builds pass)
              ├─ create GitHub Release named after tag
              ├─ attach all 5 archives
              └─ auto-generate release notes from commits
```

Each build job:
1. Builds with `cargo build --release` (or `cross` for aarch64-linux)
2. Packages the binary into the archive
3. Uploads as a workflow artifact

The `release` job downloads all artifacts and creates the GitHub Release using `softprops/action-gh-release` with `generate_release_notes: true`.

## Release Profile

Add to root `Cargo.toml`:

```toml
[profile.release]
lto = true
strip = true
codegen-units = 1
```

Produces smaller, faster binaries. Only runs on tag pushes so the slower build time is acceptable.

## Version Source of Truth

The git tag is the version source of truth. `Cargo.toml` version is a dev marker — no gating on it matching the tag since there's no crates.io publish.

## Linker Notes

No custom linker configuration needed:
- **Linux:** Rust 1.90+ defaults to LLD on x86_64-linux.
- **macOS:** Apple's `ld-prime` (shipped with Xcode 15+) is already fast.
- **Windows:** MSVC linker is the default and only practical option.

With `lto = true`, the LTO pass dominates link time anyway.

## Not In Scope

- Homebrew tap or install script
- crates.io publish
- `cargo-release` automation for version bumps
- Prerelease detection (e.g. `v0.2.0-rc1`)
- SHA256 checksums file

## Files Changed

- **New:** `.github/workflows/release.yml`
- **Edit:** `Cargo.toml` — add `[profile.release]` section
