//! Example: a minimal third-party C2 module for the Furia platform.
//!
//! This shows how a plugin developer implements a `SimulationProvider`
//! and registers it. The same pattern works for all 18 SDK traits.

use std::time::Duration;
use uuid::Uuid;

use furia_sdk::module_handle::{ModuleHandle, ModuleHealth};
use furia_sdk::simulation::{EntityState, Scenario, SimEvent, SimulationProvider};

/// Fuel consumption rate as a percentage of total fuel per hour of operation.
const FUEL_BURN_RATE_PCT_PER_HOUR: f64 = 50.0;

/// A minimal drone simulation.
pub struct SimpleDrone {
    entity_id: String,
    lat_deg: f64,
    lon_deg: f64,
    alt_m: f64,
    speed_kph: f64,
    heading_deg: f64,
    fuel_pct: f64,
    /// Accumulated simulation time in milliseconds (deterministic — no wall clock).
    sim_time_ms: u64,
    /// Module ID from the handle received at init (for lifecycle awareness).
    module_id: Option<Uuid>,
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
            sim_time_ms: 0,
            module_id: None,
        }
    }
}

impl SimulationProvider for SimpleDrone {
    fn init(&mut self, scenario: &Scenario, handle: &ModuleHandle) {
        // Read entity_id from scenario environment
        if let Some(id) = scenario
            .environment
            .get("entity_id")
            .and_then(|v| v.as_str())
        {
            self.entity_id = id.to_string();
        }
        self.module_id = Some(handle.module_id);
        tracing::info!(
            "[SimpleDrone] Simulation provider initialized (module_id={})",
            handle.module_id
        );
    }

    fn tick(&mut self, dt: Duration) -> Vec<SimEvent> {
        let dt_hours = dt.as_secs_f64() / 3600.0;
        dt.as_millis() as u64;
        self.sim_time_ms += dt_ms;

        // Move along heading (simplified great-circle approximation)
        let dist_km = self.speed_kph * dt_hours;
        self.lat_deg += dist_km * self.heading_deg.to_radians().cos() / 111.0;
        let cos_lat = self.lat_deg.to_radians().cos().max(0.01); // prevent NaN at poles
        self.lon_deg += dist_km * self.heading_deg.to_radians().sin() / (111.0 * cos_lat);

        // Consume fuel linearly
        self.fuel_pct = (self.fuel_pct - FUEL_BURN_RATE_PCT_PER_HOUR * dt_hours).max(0.0);

        let mut events = vec![];
        if self.fuel_pct < 10.0 {
            events.push(SimEvent {
                event_type: "logistics.fuel_critical".into(),
                source: self.entity_id.clone(),
                target: None,
                params: serde_json::json!({"fuel_pct": self.fuel_pct}),
                timestamp_ms: self.sim_time_ms,
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
            status: if self.fuel_pct > 0.0 {
                "alive"
            } else {
                "disabled"
            }
            .into(),
        })
    }

    fn health(&self) -> ModuleHealth {
        ModuleHealth::Healthy
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn test_init_sets_entity_id() {
        let scenario = Scenario {
            id: "test".into(),
            name: String::new(),
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
        sim.tick(Duration::from_secs(3600));
        assert!(sim.fuel_pct < 100.0);
        assert!(sim.fuel_pct >= 0.0);
    }

    #[test]
    fn test_fuel_critical_event() {
        let mut sim = SimpleDrone {
            fuel_pct: 5.0,
            ..Default::default()
        };
        let events = sim.tick(Duration::from_secs(1));
        assert!(events
            .iter()
            .any(|e| e.event_type == "logistics.fuel_critical"));
    }

    #[test]
    fn test_unknown_entity_returns_none() {
        let sim = SimpleDrone::default();
        assert!(sim.entity_state("no-such-drone").is_none());
    }
}
