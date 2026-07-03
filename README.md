# Furia Plugin Example

Complete example of a Furia C2 plugin — register custom providers, implement `InboundAdapter`/`OutboundAdapter`, and integrate with `FuriaBuilder`.

## Quickstart

```bash
cargo build
cargo run
```

## What this demonstrates

- Implementing `InboundAdapter` and `OutboundAdapter` from `furia-interop`
- Registering providers via `FuriaBuilder::with_provider()`
- Exposing provider health via `ModuleHandle`
- Structuring a plugin for the `furia-module-registry`
- Building a standalone binary that the interop-gateway can load

## Architecture

```
plugin-binary
  ├── src/main.rs          # FuriaBuilder setup + provider registration
  ├── InboundAdapter       # Parse external messages
  ├── OutboundAdapter      # Encode internal events
  └── CustomProvider       # Domain-specific logic
```

## Integration with furia-control

Add this to the interop-gateway's `Cargo.toml`:

```toml
furia-plugin-example = { git = "https://github.com/vlordier/furia-plugin-example", tag = "v0.1.0" }
```

Then register in `furia_host.rs`:

```rust
builder = builder.with_provider("custom", "my-plugin", Box::new(MyPluginProvider::new()));
```

See also: [furia-sdk-examples](https://github.com/vlordier/furia-sdk-examples) (18 example crates), [furia-core](https://github.com/vlordier/furia-core) (SDK traits).
