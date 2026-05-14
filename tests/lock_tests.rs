use std::collections::HashMap;
use envlayer::lock::EnvLock;
use envlayer::lock_guard::LockGuard;

#[test]
fn test_lock_and_is_locked() {
    let mut lock = EnvLock::new();
    assert!(!lock.is_locked("DATABASE_URL"));
    lock.lock("DATABASE_URL");
    assert!(lock.is_locked("DATABASE_URL"));
}

#[test]
fn test_lock_all() {
    let mut lock = EnvLock::new();
    lock.lock_all(["KEY_A", "KEY_B", "KEY_C"]);
    assert!(lock.is_locked("KEY_A"));
    assert!(lock.is_locked("KEY_B"));
    assert!(lock.is_locked("KEY_C"));
    assert_eq!(lock.len(), 3);
}

#[test]
fn test_unlock() {
    let mut lock = EnvLock::new();
    lock.lock("SECRET");
    assert!(lock.is_locked("SECRET"));
    let removed = lock.unlock("SECRET");
    assert!(removed);
    assert!(!lock.is_locked("SECRET"));
}

#[test]
fn test_assert_unlocked_passes_for_free_key() {
    let lock = EnvLock::new();
    assert!(lock.assert_unlocked("FREE_KEY").is_ok());
}

#[test]
fn test_assert_unlocked_fails_for_locked_key() {
    let mut lock = EnvLock::new();
    lock.lock("LOCKED_KEY");
    let result = lock.assert_unlocked("LOCKED_KEY");
    assert!(result.is_err());
    let msg = format!("{}", result.unwrap_err());
    assert!(msg.contains("LOCKED_KEY"));
}

#[test]
fn test_lock_guard_set_allowed() {
    let lock = EnvLock::new();
    let mut env: HashMap<String, String> = HashMap::new();
    let mut guard = LockGuard::new(&lock, &mut env);
    assert!(guard.set("APP_ENV", "production").is_ok());
    assert_eq!(guard.get("APP_ENV").unwrap(), "production");
}

#[test]
fn test_lock_guard_set_blocked_when_locked() {
    let mut lock = EnvLock::new();
    lock.lock("APP_ENV");
    let mut env: HashMap<String, String> = HashMap::new();
    let mut guard = LockGuard::new(&lock, &mut env);
    let result = guard.set("APP_ENV", "staging");
    assert!(result.is_err());
}

#[test]
fn test_lock_guard_remove_blocked_when_locked() {
    let mut lock = EnvLock::new();
    lock.lock("IMMUTABLE");
    let mut env: HashMap<String, String> = HashMap::from([("IMMUTABLE".into(), "yes".into())]);
    let mut guard = LockGuard::new(&lock, &mut env);
    assert!(guard.remove("IMMUTABLE").is_err());
}

#[test]
fn test_lock_guard_snapshot() {
    let lock = EnvLock::new();
    let mut env: HashMap<String, String> = HashMap::new();
    let mut guard = LockGuard::new(&lock, &mut env);
    guard.set("X", "1").unwrap();
    guard.set("Y", "2").unwrap();
    let snap = guard.snapshot();
    assert_eq!(snap.len(), 2);
    assert_eq!(snap["X"], "1");
}
