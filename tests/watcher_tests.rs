use std::io::Write;
use std::path::PathBuf;
use tempfile::NamedTempFile;
use envlayer::watcher::Watcher;

fn make_temp_file(content: &str) -> NamedTempFile {
    let mut f = NamedTempFile::new().expect("tempfile");
    write!(f, "{}", content).expect("write");
    f
}

#[test]
test_watch_registers_file() {
    let f = make_temp_file("KEY=val");
    let mut w = Watcher::new();
    w.watch(f.path());
    assert_eq!(w.watch_count(), 1);
    assert!(w.is_watching(f.path()));
}

#[test]
fn test_unwatch_removes_file() {
    let f = make_temp_file("KEY=val");
    let mut w = Watcher::new();
    w.watch(f.path());
    w.unwatch(f.path());
    assert_eq!(w.watch_count(), 0);
    assert!(!w.is_watching(f.path()));
}

#[test]
fn test_no_change_returns_empty() {
    let f = make_temp_file("KEY=val");
    let mut w = Watcher::new();
    w.watch(f.path());
    // Poll once to baseline
    let _ = w.changed_files().unwrap();
    // Poll again — no write occurred
    let changed = w.changed_files().unwrap();
    assert!(changed.is_empty());
}

#[test]
fn test_change_detected_after_write() {
    let mut f = make_temp_file("KEY=val");
    let mut w = Watcher::new();
    w.watch(f.path());
    // Baseline
    let _ = w.changed_files().unwrap();
    // Modify the file
    std::thread::sleep(std::time::Duration::from_millis(10));
    write!(f, "\nOTHER=1").expect("write");
    f.flush().expect("flush");
    let changed = w.changed_files().unwrap();
    assert_eq!(changed.len(), 1);
}

#[test]
fn test_watch_nonexistent_file_no_panic() {
    let mut w = Watcher::new();
    w.watch("/tmp/__envlayer_nonexistent_9999.env");
    assert_eq!(w.watch_count(), 1);
    let changed = w.changed_files().unwrap();
    // No mtime → no change from None to None
    assert!(changed.is_empty());
}
