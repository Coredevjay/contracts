use soroban_sdk::{contracttype, Address, BytesN};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PendingUnstake {
    pub amount: i128,
    /// Ledger timestamp at which `amount` becomes withdrawable.
    pub unlock_at: u64,
}

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    StakeToken,
    /// The only address allowed to call notify_reward (the fee splitter).
    Splitter,
    Cooldown,
    TotalStaked,
    /// Every reward token ever notified, in first-seen order.
    RewardTokens,
    /// Cumulative reward per staked unit for a token, scaled by
    /// ACC_PRECISION.
    RewardPerToken(Address),
    /// Rewards notified while nothing was staked, carried over to the next
    /// time there is stake to distribute them to.
    Undistributed(Address),
    Stake(Address),
    /// RewardPerToken(token) as of `user`'s last accounting update.
    UserPaid(Address, Address),
    /// Rewards accrued to `user` in `token` but not yet claimed.
    Owed(Address, Address),
    PendingUnstake(Address),
    /// Sum of `user`'s live commitments; that much stake cannot be unstaked.
    Committed(Address),
    /// `(user, committer, key)` -> Commitment.
    Commitment(Address, Address, BytesN<32>),
    /// Incidents already slashed, per committer; prevents a double slash.
    Slashed(Address, BytesN<32>),
}

/// Stake locked on behalf of an external contract (e.g. a price_oracle
/// feeder slot). The committer is identified only by an opaque
/// `(contract, key)` pair, so staking never imports oracle types.
/// The commitment is per slot, not global: one user can back several.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Commitment {
    pub amount: i128,
    /// 0 while active; otherwise when `finalize_release` becomes callable.
    pub release_at: u64,
}
