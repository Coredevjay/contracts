//! Reentrancy guard and checks-effects-interactions (CEI) rules.
//!
//! The flag lives in **instance** storage under a caller-supplied key (one
//! key per guarded entrypoint class), so it shares the contract's TTL and
//! can never be stranded by a stray persistent entry. A check is a single
//! storage read. Re-entry fails closed with the caller's own error.
//!
//! Soroban has no exceptions: a failed invocation rolls back all storage
//! writes, including a held flag, so a panic inside a token or cross-call
//! cannot leave the guard stuck. Early `return`s are the only path that
//! can leak it, which is why [`with_guard`] releases on every return.
//!
//! CEI checklist for guarded entrypoints:
//! 1. `acquire` the guard.
//! 2. Authorize and validate every input — before any call out.
//! 3. Mutate local state (balances, tags, positions).
//! 4. Only then make the external call (token transfer, cross-contract).
//! 5. `release` on every return path (use [`with_guard`]).
//!
//! The guard takes no `Auth` argument, so it does not change the
//! `Auth::Multisig` call shape of guarded entrypoints.

use soroban_sdk::{panic_with_error, Env, IntoVal, Val};

/// True while `key`'s guard is held.
pub fn is_entered<K: IntoVal<Env, Val>>(env: &Env, key: &K) -> bool {
    env.storage().instance().get(key).unwrap_or(false)
}

/// Takes the guard, panicking with `err` if it is already held.
pub fn enter<K, E>(env: &Env, key: &K, err: E)
where
    K: IntoVal<Env, Val>,
    E: Into<soroban_sdk::Error>,
{
    if is_entered(env, key) {
        panic_with_error!(env, err);
    }
    env.storage().instance().set(key, &true);
}

/// Releases the guard.
pub fn exit<K: IntoVal<Env, Val>>(env: &Env, key: &K) {
    env.storage().instance().remove(key);
}

/// Runs `f` with the guard held and releases it on return, whatever `f`
/// returns (including `Err` values).
pub fn with_guard<K, E, R>(env: &Env, key: &K, err: E, f: impl FnOnce() -> R) -> R
where
    K: IntoVal<Env, Val>,
    E: Into<soroban_sdk::Error>,
{
    enter(env, key, err);
    let out = f();
    exit(env, key);
    out
}
