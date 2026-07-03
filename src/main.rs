//! Entry point for the Furia plugin example.
//!
//! Creates a `FuriaBuilder`, registers the `SimpleDrone` simulation provider,
//! and starts the platform.

use furia_platform::FuriaBuilder;

use furia_plugin_example::SimpleDrone;

fn main() {
    let platform = FuriaBuilder::new()
        .with_provider("simulation", "simple-drone", Box::new(SimpleDrone::default()))
        .without_builtins()
        .build();

    println!("Furia platform built with {} providers", platform.provider_list().len());
    for (kind, name) in platform.provider_list() {
        println!("  Provider: {}/{}", kind, name);
    }
    platform.run();
}