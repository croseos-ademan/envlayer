//! Audit trail for environment variable changes across layers.

use std::collections::VecDeque;
use std::time::{SystemTime, UNIX_EPOCH};

/// The kind of operation recorded in the audit log.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuditEvent {
    Set { key: String, value: String, layer: String },
    Override { key: String, old_value: String, new_value: String, layer: String },
    Remove { key: String, layer: String },
    Merge { source_layer: String, target_layer: String, key_count: usize },
}

/// A single entry in the audit log.
#[derive(Debug, Clone)]
pub struct AuditEntry {
    pub event: AuditEvent,
    pub timestamp_ms: u64,
}

impl AuditEntry {
    pub fn new(event: AuditEvent) -> Self {
        let timestamp_ms = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_millis() as u64)
            .unwrap_or(0);
        Self { event, timestamp_ms }
    }
}

/// Bounded audit log that records environment variable lifecycle events.
#[derive(Debug, Clone)]
pub struct AuditLog {
    entries: VecDeque<AuditEntry>,
    capacity: usize,
}

impl AuditLog {
    pub fn new(capacity: usize) -> Self {
        Self {
            entries: VecDeque::with_capacity(capacity),
            capacity,
        }
    }

    pub fn record(&mut self, event: AuditEvent) {
        if self.entries.len() == self.capacity {
            self.entries.pop_front();
        }
        self.entries.push_back(AuditEntry::new(event));
    }

    pub fn entries(&self) -> impl Iterator<Item = &AuditEntry> {
        self.entries.iter()
    }

    pub fn entries_for_key<'a>(&'a self, key: &'a str) -> impl Iterator<Item = &'a AuditEntry> {
        self.entries.iter().filter(move |e| match &e.event {
            AuditEvent::Set { key: k, .. } => k == key,
            AuditEvent::Override { key: k, .. } => k == key,
            AuditEvent::Remove { key: k, .. } => k == key,
            AuditEvent::Merge { .. } => false,
        })
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }
}

impl Default for AuditLog {
    fn default() -> Self {
        Self::new(1024)
    }
}
