// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-omarchy-power: omarchy power component model.
//!
//! Real implementation: powers, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Power {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OmarchyPowerError {
    EmptyPowerName,
    DuplicatePower(String),
    EmptyEventSubject,
}

impl fmt::Display for OmarchyPowerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyPowerName => write!(f, "power name must not be empty"),
            Self::DuplicatePower(id) => write!(f, "duplicate power {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for OmarchyPowerError {}

pub struct OmarchyPower {
    pub name: String,
    powers: BTreeMap<String, Power>,
    events: BTreeMap<String, Event>,
}

impl OmarchyPower {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            powers: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_power(&mut self, item: Power) -> Result<(), OmarchyPowerError> {
        if item.name.trim().is_empty() {
            return Err(OmarchyPowerError::EmptyPowerName);
        }
        let id = item.id.clone();
        if self.powers.contains_key(&id) {
            return Err(OmarchyPowerError::DuplicatePower(id.clone()));
        }
        self.powers.insert(id, item);
        Ok(())
    }

    pub fn power_count(&self) -> usize {
        self.powers.len()
    }

    pub fn get_power(&self, id: &str) -> Option<&Power> {
        self.powers.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), OmarchyPowerError> {
        if event.subject.trim().is_empty() {
            return Err(OmarchyPowerError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_power(id: &str, name: &str) -> Power {
        Power {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = OmarchyPower::new("OmarchyPower");
        assert_eq!(m.name, "OmarchyPower");
        assert_eq!(m.power_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_power() {
        let mut m = OmarchyPower::new("OmarchyPower");
        m.add_power(sample_power("a1", "Alpha")).unwrap();
        assert_eq!(m.power_count(), 1);
        assert!(m.get_power("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = OmarchyPower::new("OmarchyPower");
        assert!(m.add_power(sample_power("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = OmarchyPower::new("OmarchyPower");
        m.add_power(sample_power("x", "One")).unwrap();
        assert!(m.add_power(sample_power("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = OmarchyPower::new("OmarchyPower");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
