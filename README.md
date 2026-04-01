# XENØr Sim

Simulation playground for XENØr protocol mechanics.

`xenor-sim` is the validation layer in the XENØr stack. It runs small experiments on top of `xenor-core` so propagation rules and incentive behavior can be checked before they harden into public claims.

## Focus

- scenario validation on top of `xenor-core`
- deterministic behavior checks
- propagation and incentive experiments
- quick executable prototypes

## Related Repositories

- [`xenor-core`](https://github.com/XENOr-god/xenor-core) — reusable execution and propagation primitives
- [`xenor-site`](https://github.com/XENOr-god/xenor-site) — canonical public surface for architecture and repository context
- [`xenor-sale`](https://github.com/XENOr-god/xenor-sale) — archived experiment kept as historical launch research

## Quick Start

```bash
cargo check
cargo run
```

## Status

Executable sandbox and experiment runner. Scenarios may change quickly as research evolves.
