# xenor-sim

`xenor-sim` is the scenario and validation layer in the XENOr stack. It exists
to run deterministic experiments around `xenor-core` so propagation behavior,
activation rules, and incentive assumptions can be exercised before they harden
into public claims or broader integrations.

## Status

Active, experimental validation repository. This repo is useful today, but it
is still research-stage and is not yet versioned as a stable public release.

## Why This Repo Exists

`xenor-sim` keeps scenario execution and validation separate from the reusable
execution rules in `xenor-core`. That separation makes it easier to evolve
experiments quickly without muddying the deterministic core layer.

## Relationship to the XENOr Stack

- `xenor-core` supplies the deterministic execution primitives used here
- `xenor-sim` is where scenario runs and validation experiments live
- `xenor-site` explains the public stack and should be the first repo most
  newcomers open
- `xenor-engine` is a separate deterministic engine substrate for replay,
  snapshot, and systems-level work

## Quick Start / Local Development

Toolchain: stable Rust with edition 2024 support.

```bash
cargo check
cargo run
```

- `cargo run` executes the current scenario runner in
  [`src/main.rs`](src/main.rs).
- `xenor-sim` depends on `xenor-core` and is best read alongside it.

## Repository Boundaries / Non-goals

- This is not the authoritative execution/core systems repo. Use `xenor-core`
  for that.
- This is not the canonical public surface. Use `xenor-site` for that.
- This is not the sale or launch path.
- Expect scenarios to evolve faster here than the underlying core contracts.

## Related Repositories

- [`xenor-core`](https://github.com/XENOr-god/xenor-core) — deterministic
  execution/core systems layer
- [`xenor-site`](https://github.com/XENOr-god/xenor-site) — canonical public
  surface and repository map
- [`xenor-engine`](https://github.com/XENOr-god/xenor-engine) — deterministic
  engine and replay/snapshot substrate
- [`xenor-sale`](https://github.com/XENOr-god/xenor-sale) — archived
  historical sale prototype

## License

This repository does not currently publish a separate license file.
