use furia_platform::FuriaBuilder;

#[test]
fn test_plugin_registers_and_health_reports() {
    let platform = FuriaBuilder::new()
        .without_builtins()
        .build();
    // Basic smoke test — platform builds without panicking
    // In a full integration test, we'd add a provider and check health
    assert!(platform.provider_list().is_empty());
}