//! Formats and reports audit log contents for inspection and export.

use crate::audit::{AuditEntry, AuditEvent, AuditLog};

/// Output format for audit reports.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReportFormat {
    Plain,
    Csv,
}

/// Generates textual reports from an [`AuditLog`].
pub struct AuditReporter<'a> {
    log: &'a AuditLog,
}

impl<'a> AuditReporter<'a> {
    pub fn new(log: &'a AuditLog) -> Self {
        Self { log }
    }

    pub fn report(&self, format: ReportFormat) -> String {
        match format {
            ReportFormat::Plain => self.plain_report(),
            ReportFormat::Csv => self.csv_report(),
        }
    }

    fn plain_report(&self) -> String {
        let mut out = String::new();
        for entry in self.log.entries() {
            out.push_str(&format!("[{}ms] {}\n", entry.timestamp_ms, describe_event(&entry.event)));
        }
        out
    }

    fn csv_report(&self) -> String {
        let mut out = String::from("timestamp_ms,event_type,key,layer,detail\n");
        for entry in self.log.entries() {
            let (event_type, key, layer, detail) = csv_fields(&entry.event);
            out.push_str(&format!("{},{},{},{}\n",
                entry.timestamp_ms, event_type,
                csv_escape(&format!("{},{}", key, layer)),
                csv_escape(detail)
            ));
        }
        out
    }
}

fn describe_event(event: &AuditEvent) -> String {
    match event {
        AuditEvent::Set { key, value, layer } =>
            format!("SET {key}={value} (layer: {layer})"),
        AuditEvent::Override { key, old_value, new_value, layer } =>
            format!("OVERRIDE {key}: {old_value} -> {new_value} (layer: {layer})"),
        AuditEvent::Remove { key, layer } =>
            format!("REMOVE {key} (layer: {layer})"),
        AuditEvent::Merge { source_layer, target_layer, key_count } =>
            format!("MERGE {source_layer} -> {target_layer} ({key_count} keys)"),
    }
}

fn csv_fields(event: &AuditEvent) -> (&str, &str, &str, &str) {
    match event {
        AuditEvent::Set { key, layer, .. } => ("set", key, layer, ""),
        AuditEvent::Override { key, layer, .. } => ("override", key, layer, ""),
        AuditEvent::Remove { key, layer } => ("remove", key, layer, ""),
        AuditEvent::Merge { source_layer, target_layer, .. } => ("merge", source_layer, target_layer, ""),
    }
}

fn csv_escape(s: &str) -> &str {
    s
}
