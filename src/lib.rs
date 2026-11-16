// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-gnu-gettext: gnu gettext component model.
//!
//! Real implementation: gettexts, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Gettext {
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
pub enum GnuGettextError {
    EmptyGettextName,
    DuplicateGettext(String),
    EmptyEventSubject,
}

impl fmt::Display for GnuGettextError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyGettextName => write!(f, "gettext name must not be empty"),
            Self::DuplicateGettext(id) => write!(f, "duplicate gettext {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for GnuGettextError {}

pub struct GnuGettext {
    pub name: String,
    gettexts: BTreeMap<String, Gettext>,
    events: BTreeMap<String, Event>,
}

impl GnuGettext {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            gettexts: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_gettext(&mut self, item: Gettext) -> Result<(), GnuGettextError> {
        if item.name.trim().is_empty() {
            return Err(GnuGettextError::EmptyGettextName);
        }
        let id = item.id.clone();
        if self.gettexts.contains_key(&id) {
            return Err(GnuGettextError::DuplicateGettext(id.clone()));
        }
        self.gettexts.insert(id, item);
        Ok(())
    }

    pub fn gettext_count(&self) -> usize {
        self.gettexts.len()
    }

    pub fn get_gettext(&self, id: &str) -> Option<&Gettext> {
        self.gettexts.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), GnuGettextError> {
        if event.subject.trim().is_empty() {
            return Err(GnuGettextError::EmptyEventSubject);
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

    fn sample_gettext(id: &str, name: &str) -> Gettext {
        Gettext {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = GnuGettext::new("GnuGettext");
        assert_eq!(m.name, "GnuGettext");
        assert_eq!(m.gettext_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_gettext() {
        let mut m = GnuGettext::new("GnuGettext");
        m.add_gettext(sample_gettext("a1", "Alpha")).unwrap();
        assert_eq!(m.gettext_count(), 1);
        assert!(m.get_gettext("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = GnuGettext::new("GnuGettext");
        assert!(m.add_gettext(sample_gettext("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = GnuGettext::new("GnuGettext");
        m.add_gettext(sample_gettext("x", "One")).unwrap();
        assert!(m.add_gettext(sample_gettext("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = GnuGettext::new("GnuGettext");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
