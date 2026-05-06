use std::io::Write;
use std::sync::{Arc, Mutex};
use tempfile::NamedTempFile;
use envlayer::watch_trigger::WatchTrigger;

fn make_temp_file(content: &str) -> NamedTempFile {
    let mut f = NamedTempFile::new().expect("tempfile");
    write!(f, "{}", content).expect("write");
    f
}

#[test]
fn test_trigger_no_callback_no_panic() {
    let f = make_temp_file("A=1");
    let mut t = WatchTrigger::new();
    t.watch(f.path());
    let count = t.poll().expect("poll");
    // First poll baselines — no prior state, so may or may not detect.
    let _ = count;
}

#[test]
fn test_trigger_callback_invoked_on_change() {
    let mut f = make_temp_file("A=1");
    let triggered: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(vec![]));
    let triggered_clone = Arc::clone(&triggered);

    let mut t = WatchTrigger::new().on_change(move |p| {
        triggered_clone
            .lock()
            .unwrap()
            .push(p.to_string_lossy().to_string());
    });

    t.watch(f.path());
    // Baseline poll
    let _ = t.poll().unwrap();

    // Mutate file
    std::thread::sleep(std::time::Duration::from_millis(10));
    write!(f, "\nB=2").unwrap();
    f.flush().unwrap();

    let n = t.poll().unwrap();
    assert_eq!(n, 1);
    assert_eq!(triggered.lock().unwrap().len(), 1);
}

#[test]
fn test_watch_count_reflects_registrations() {
    let f1 = make_temp_file("X=1");
    let f2 = make_temp_file("Y=2");
    let mut t = WatchTrigger::default();
    t.watch(f1.path());
    t.watch(f2.path());
    assert_eq!(t.watch_count(), 2);
}

#[test]
fn test_no_change_callback_not_invoked() {
    let f = make_temp_file("Z=3");
    let calls: Arc<Mutex<u32>> = Arc::new(Mutex::new(0));
    let calls_clone = Arc::clone(&calls);

    let mut t = WatchTrigger::new().on_change(move |_| {
        *calls_clone.lock().unwrap() += 1;
    });

    t.watch(f.path());
    let _ = t.poll().unwrap();
    let _ = t.poll().unwrap();
    // No write occurred after baseline
    assert_eq!(*calls.lock().unwrap(), 0);
}
