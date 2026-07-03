# Furia Plugin Example

Complete example of a Furia C2 plugin — implement a custom `SimulationProvider` and register it via `FuriaBuilder`.

## Quickstart

```bash
cargo build
cargo run
```

## What this demonstrates

- Implementing `SimulationProvider`
- Registering providers via `FuriaBuilder::with_provider()`
- Exposing provider health via `ModuleHandle`
- Structuring a plugin for the `furia-module-registry`

## Architecture

```
plugin-binary
  ├── src/main.rs          # FuriaBuilder setup + provider registration
  └── SimpleDrone          # SimulationProvider implementation
```

## Integration with furia-control

Add this to the interop-gateway's `Cargo.toml`:

```toml
furia-plugin-example = { git = "https://github.com/vlordier/furia-plugin-example", tag = "v0.1.0" }
```

Then register in `furia_host.rs`:

```rust
builder = builder.with_provider("simulation", "simple-drone", Box::new(SimpleDrone::new()));
```

See also: [furia-sdk-examples](https://github.com/vlordier/furia-sdk-examples) (18 example crates), [furia-core](https://github.com/vlordier/furia-core) (SDK traits).