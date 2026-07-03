//! Example: a minimal third-party C2 module for the Furia platform.
//!
//! This shows how a plugin developer implements a `SimulationProvider`
//! and registers it. The same pattern works for all 18 SDK traits.

use std::time::Duration;

use furia_platform::FuriaBuilder;
use furia_sdk::module_handle::{ModuleHandle, ModuleHealth};
use furia_sdk::simulation::{EntityState, Scenario, SimEvent, SimulationProvider};

/// A minimal drone simulation.
pub struct SimpleDrone {
    entity_id: String,
    lat_deg: f64,
    lon_deg: f64,
    alt_m: f64,
    speed_kph: f64,
    heading_deg: f64,
    fuel_pct: f64,
}

impl Default for SimpleDrone {
    fn default() -> Self {
        Self {
            entity_id: "drone-001".into(),
            lat_deg: 48.8566,
            lon_deg: 2.3522,
            alt_m: 500.0,
            speed_kph: 80.0,
            heading_deg: 45.0,
            fuel_pct: 100.0,
        }
    }
}

impl SimulationProvider for SimpleDrone {
    fn init(&mut self, scenario: &Scenario, _handle: &ModuleHandle) {
        // Read entity_id from scenario environment
        if let Some(id) = scenario.environment.get("entity_id").and_then(|v| v.as_str()) {
            self.entity_id = id.to_string();
        }
    }

    fn tick(&mut self, dt: Duration) -> Vec<SimEvent> {
        let dt_hours = dt.as_secs_f64() / 3600.0;

        // Move along heading (simplified great-circle approximation)
        let dist_km = self.speed_kph * dt_hours;
        self.lat_deg += dist_km * self.heading_deg.to_radians().cos() / 111.0;
        self.lon_deg += dist_km * self.heading_deg.to_radians().sin()
            / (111.0 * self.lat_deg.to_radians().cos());

        // Consume fuel linearly
        self.fuel_pct = (self.fuel_pct - 0.5 * dt_hours * 100.0).max(0.0);

        let mut events = vec![];
        if self.fuel_pct < 10.0 {
            events.push(SimEvent {
                event_type: "logistics.fuel_critical".into(),
                source: self.entity_id.clone(),
                target: None,
                params: serde_json::json!({"fuel_pct": self.fuel_pct}),
                timestamp_ms: chrono::Utc::now().timestamp_millis() as u64,
            });
        }
        events
    }

    fn entity_state(&self, entity_id: &str) -> Option<EntityState> {
        if entity_id != self.entity_id {
            return None;
        }
        Some(EntityState {
            entity_id: self.entity_id.clone(),
            position: (self.lat_deg, self.lon_deg, self.alt_m),
            velocity: Some(self.speed_kph),
            heading: Some(self.heading_deg),
            status: if self.fuel_pct > 0.0 { "alive" } else { "disabled" }.into(),
        })
    }

    fn health(&self) -> ModuleHealth {
        ModuleHealth::Healthy
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn test_init_sets_entity_id() {
        let scenario = Scenario {
            id: "test".into(),
            name: "".into(),
            duration_secs: 3600,
            order_of_battle: serde_json::Value::Null,
            timeline: vec![],
            environment: serde_json::json!({"entity_id": "my-drone"}),
        };
        let mut sim = SimpleDrone::default();
        sim.init(&scenario, &ModuleHandle::new_test(Uuid::new_v4()));
        assert_eq!(sim.entity_id, "my-drone");
    }

    #[test]
    fn test_tick_burns_fuel() {
        let mut sim = SimpleDrone::default();
        sim.init(
            &Scenario {
                id: String::new(),
                name: String::new(),
                duration_secs: 0,
                order_of_battle: serde_json::Value::Null,
                timeline: vec![],
                environment: serde_json::Value::Null,
            },
            &ModuleHandle::new_test(Uuid::new_v4()),
        );
        sim.tick(Duration::from_secs(3600));
        assert!(sim.fuel_pct < 100.0);
        assert!(sim.fuel_pct >= 0.0);
    }

    #[test]
    fn test_fuel_critical_event() {
        let mut sim = SimpleDrone::default();
        sim.fuel_pct = 5.0;
        let events = sim.tick(Duration::from_secs(1));
        assert!(events.iter().any(|e| e.event_type == "logistics.fuel_critical"));
    }

    #[test]
    fn test_unknown_entity_returns_none() {
        let sim = SimpleDrone::default();
        assert!(sim.entity_state("no-such-drone").is_none());
    }
}