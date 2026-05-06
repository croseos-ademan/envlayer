use envlayer::audit::{AuditEvent, AuditLog};
use envlayer::audit_reporter::{AuditReporter, ReportFormat};

fn make_log() -> AuditLog {
    let mut log = AuditLog::new(20);
    log.record(AuditEvent::Set {
        key: "DB_URL".into(),
        value: "postgres://localhost".into(),
        layer: "base".into(),
    });
    log.record(AuditEvent::Override {
        key: "DB_URL".into(),
        old_value: "postgres://localhost".into(),
        new_value: "postgres://prod".into(),
        layer: "ci".into(),
    });
    log.record(AuditEvent::Remove {
        key: "DEBUG".into(),
        layer: "ci".into(),
    });
    log
}

#[test]
fn test_plain_report_contains_set() {
    let log = make_log();
    let reporter = AuditReporter::new(&log);
    let report = reporter.report(ReportFormat::Plain);
    assert!(report.contains("SET DB_URL"));
    assert!(report.contains("layer: base"));
}

#[test]
fn test_plain_report_contains_override() {
    let log = make_log();
    let reporter = AuditReporter::new(&log);
    let report = reporter.report(ReportFormat::Plain);
    assert!(report.contains("OVERRIDE DB_URL"));
    assert!(report.contains("postgres://localhost -> postgres://prod"));
}

#[test]
fn test_plain_report_contains_remove() {
    let log = make_log();
    let reporter = AuditReporter::new(&log);
    let report = reporter.report(ReportFormat::Plain);
    assert!(report.contains("REMOVE DEBUG"));
}

#[test]
fn test_csv_report_has_header() {
    let log = make_log();
    let reporter = AuditReporter::new(&log);
    let report = reporter.report(ReportFormat::Csv);
    assert!(report.starts_with("timestamp_ms,event_type,key,layer,detail"));
}

#[test]
fn test_csv_report_has_entries() {
    let log = make_log();
    let reporter = AuditReporter::new(&log);
    let report = reporter.report(ReportFormat::Csv);
    let lines: Vec<&str> = report.lines().collect();
    // header + 3 entries
    assert_eq!(lines.len(), 4);
}

#[test]
fn test_empty_log_plain_report() {
    let log = AuditLog::new(10);
    let reporter = AuditReporter::new(&log);
    let report = reporter.report(ReportFormat::Plain);
    assert!(report.is_empty());
}
