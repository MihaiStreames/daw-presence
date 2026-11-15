use super::*;

#[test]
fn ram_switches_to_gb_at_1024_mb() {
    let below = DawStatus {
        memory_mb: 1023,
        ..Default::default()
    };

    let at = DawStatus {
        memory_mb: 1024,
        ..below.clone()
    };

    assert_eq!(below.ram_usage_str(), "1023MB");
    assert_eq!(at.ram_usage_str(), "1.00GB");
}
