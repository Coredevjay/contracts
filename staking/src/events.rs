use soroban_sdk::{Address, Env, Symbol};

pub fn staked(env: &Env, user: Address, amount: i128) {
    env.events()
        .publish((Symbol::new(env, "staked"), user), amount);
}

pub fn unstake_requested(env: &Env, user: Address, amount: i128, unlock_at: u64) {
    env.events().publish(
        (Symbol::new(env, "unstake_requested"), user),
        (amount, unlock_at),
    );
}

pub fn withdrawn(env: &Env, user: Address, amount: i128) {
    env.events()
        .publish((Symbol::new(env, "withdrawn"), user), amount);
}

pub fn reward_notified(env: &Env, token: Address, amount: i128) {
    env.events()
        .publish((Symbol::new(env, "reward_notified"), token), amount);
}

pub fn reward_claimed(env: &Env, user: Address, token: Address, amount: i128) {
    env.events()
        .publish((Symbol::new(env, "reward_claimed"), user, token), amount);
}

pub fn committed(env: &Env, user: Address, committer: Address, amount: i128) {
    env.events()
        .publish((Symbol::new(env, "committed"), user, committer), amount);
}

pub fn release_started(env: &Env, user: Address, committer: Address, release_at: u64) {
    env.events().publish(
        (Symbol::new(env, "release_started"), user, committer),
        release_at,
    );
}

pub fn released(env: &Env, user: Address, committer: Address, amount: i128) {
    env.events()
        .publish((Symbol::new(env, "released"), user, committer), amount);
}

/// `from_unbonding` + `from_stake` == `amount`.
pub fn slashed(
    env: &Env,
    user: Address,
    committer: Address,
    reason: Symbol,
    amount: i128,
    from_unbonding: i128,
) {
    env.events().publish(
        (Symbol::new(env, "slashed"), user, committer),
        (reason, amount, from_unbonding, amount - from_unbonding),
    );
}
