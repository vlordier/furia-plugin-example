use furia_platform::FuriaBuilder;
use furia_plugin_example::SimpleDrone;

#[test]
fn test_plugin_registers_provider() {
    let platform = FuriaBuilder::new()
        .with_provider(
            "simulation",
            "simple-drone",
            Box::new(SimpleDrone::default()),
        )
        .without_builtins()
        .build();

    let providers = platform.provider_list();
    let has_simulation = providers
        .iter()
        .any(|(kind, name)| kind == "simulation" && name == "simple-drone");
    assert!(
        has_simulation,
        "Expected simulation:simple-drone to be registered"
    );
}
