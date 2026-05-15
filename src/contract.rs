use alloc::vec::Vec;
use alloy_primitives::{Address, U256};
use stylus_sdk::{evm, msg};

use crate::{events, Error, WaveEscrow};

// ---------------------------------------------------------------------------
// Access control helpers
// ---------------------------------------------------------------------------

fn only_owner(storage: &WaveEscrow) -> Result<(), Vec<u8>> {
    if msg::sender() != storage.owner.get() {
        return Err(Error::NotOwner.into());
    }
    Ok(())
}

fn only_oracle(storage: &WaveEscrow) -> Result<(), Vec<u8>> {
    if msg::sender() != storage.wave_oracle.get() {
        return Err(Error::NotOracle.into());
    }
    Ok(())
}

fn not_paused(storage: &WaveEscrow) -> Result<(), Vec<u8>> {
    if storage.paused.get() {
        return Err(Error::ContractPaused.into());
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Wave lifecycle
// ---------------------------------------------------------------------------

pub fn initialize_wave(
    storage: &mut WaveEscrow,
    wave_id: U256,
    budget: U256,
    end_timestamp: U256,
) -> Result<(), Vec<u8>> {
    only_owner(storage)?;
    not_paused(storage)?;

    if budget == U256::ZERO {
        return Err(Error::ZeroBudget.into());
    }
    if end_timestamp <= evm::timestamp() {
        return Err(Error::InvalidTimestamp.into());
    }

    let existing = storage.waves.getter(wave_id);
    if existing.total_budget.get() != U256::ZERO {
        return Err(Error::WaveAlreadyExists.into());
    }

    let wave = storage.waves.setter(wave_id);
    wave.total_budget.set(budget);
    wave.end_timestamp.set(end_timestamp);
    wave.points_finalized.set(false);
    wave.total_points.set(U256::ZERO);
    wave.total_claimed.set(U256::ZERO);

    events::wave_initialized(wave_id, budget, end_timestamp, msg::sender());
    Ok(())
}

pub fn finalize_wave_points(
    storage: &mut WaveEscrow,
    wave_id: U256,
    total_points: U256,
) -> Result<(), Vec<u8>> {
    only_oracle(storage)?;
    not_paused(storage)?;

    if total_points == U256::ZERO {
        return Err(Error::ZeroPoints.into());
    }

    let wave = storage.waves.getter(wave_id);
    if wave.total_budget.get() == U256::ZERO {
        return Err(Error::WaveNotFound.into());
    }
    if wave.points_finalized.get() {
        return Err(Error::WaveAlreadyFinalized.into());
    }

    storage.waves.setter(wave_id).total_points.set(total_points);
    storage.waves.setter(wave_id).points_finalized.set(true);

    events::points_finalized(wave_id, total_points, msg::sender());
    Ok(())
}

pub fn deposit_funds(
    storage: &mut WaveEscrow,
    wave_id: U256,
    amount: U256,
) -> Result<(), Vec<u8>> {
    not_paused(storage)?;

    let wave = storage.waves.getter(wave_id);
    if wave.total_budget.get() == U256::ZERO {
        return Err(Error::WaveNotFound.into());
    }

    let current = wave.total_budget.get();
    storage.waves.setter(wave_id).total_budget.set(current + amount);

    events::funds_deposited(wave_id, msg::sender(), amount);
    Ok(())
}

pub fn claim_reward(
    storage: &mut WaveEscrow,
    wave_id: U256,
    contributor_points: U256,
) -> Result<(), Vec<u8>> {
    not_paused(storage)?;

    let wave = storage.waves.getter(wave_id);
    if wave.total_budget.get() == U256::ZERO {
        return Err(Error::WaveNotFound.into());
    }
    if !wave.points_finalized.get() {
        return Err(Error::WaveNotFinalized.into());
    }
    if contributor_points == U256::ZERO {
        return Err(Error::ZeroPoints.into());
    }

    let contributor = msg::sender();
    if storage.has_claimed.getter(wave_id).getter(contributor) {
        return Err(Error::AlreadyClaimed.into());
    }

    let total_points = wave.total_points.get();
    let budget = wave.total_budget.get();
    let reward = budget * contributor_points / total_points;

    if reward == U256::ZERO {
        return Err(Error::ZeroReward.into());
    }

    let claimed = wave.total_claimed.get();
    let remaining = budget - claimed;
    if reward > remaining {
        return Err(Error::InsufficientBalance.into());
    }

    // checks-effects-interactions
    storage.has_claimed.setter(wave_id).setter(contributor).set(true);
    storage.waves.setter(wave_id).total_claimed.set(claimed + reward);

    events::reward_claimed(wave_id, contributor, reward);
    Ok(())
}

// ---------------------------------------------------------------------------
// Ownership & Oracle
// ---------------------------------------------------------------------------

pub fn transfer_ownership(
    storage: &mut WaveEscrow,
    new_owner: Address,
) -> Result<(), Vec<u8>> {
    only_owner(storage)?;

    if new_owner == Address::ZERO {
        return Err(Error::ZeroAddress.into());
    }

    let old = storage.owner.get();
    storage.owner.set(new_owner);
    events::ownership_transferred(old, new_owner);
    Ok(())
}

pub fn update_oracle(
    storage: &mut WaveEscrow,
    new_oracle: Address,
) -> Result<(), Vec<u8>> {
    only_owner(storage)?;

    if new_oracle == Address::ZERO {
        return Err(Error::ZeroAddress.into());
    }

    let old = storage.wave_oracle.get();
    storage.wave_oracle.set(new_oracle);
    events::oracle_updated(old, new_oracle);
    Ok(())
}

// ---------------------------------------------------------------------------
// Pausability
// ---------------------------------------------------------------------------

pub fn pause(storage: &mut WaveEscrow) -> Result<(), Vec<u8>> {
    only_owner(storage)?;

    if storage.paused.get() {
        return Ok(());
    }

    storage.paused.set(true);
    events::paused(msg::sender());
    Ok(())
}

pub fn unpause(storage: &mut WaveEscrow) -> Result<(), Vec<u8>> {
    only_owner(storage)?;

    if !storage.paused.get() {
        return Ok(());
    }

    storage.paused.set(false);
    events::unpaused(msg::sender());
    Ok(())
}

// ---------------------------------------------------------------------------
// Emergency
// ---------------------------------------------------------------------------

pub fn emergency_withdraw(
    storage: &mut WaveEscrow,
    wave_id: U256,
    to: Address,
) -> Result<(), Vec<u8>> {
    only_owner(storage)?;

    if !storage.paused.get() {
        return Err(Error::ContractPaused.into());
    }

    let wave = storage.waves.getter(wave_id);
    if wave.total_budget.get() == U256::ZERO {
        return Err(Error::WaveNotFound.into());
    }

    let budget = wave.total_budget.get();
    let claimed = wave.total_claimed.get();
    let withdrawable = budget - claimed;

    if withdrawable == U256::ZERO || to == Address::ZERO {
        return Err(Error::InsufficientBalance.into());
    }

    storage.waves.setter(wave_id).total_budget.set(claimed);

    events::emergency_withdraw(wave_id, to, withdrawable);
    Ok(())
}

// ---------------------------------------------------------------------------
// View functions
// ---------------------------------------------------------------------------

pub fn get_wave(
    storage: &WaveEscrow,
    wave_id: U256,
) -> Result<(U256, U256, bool, U256, U256), Vec<u8>> {
    let wave = storage.waves.getter(wave_id);
    if wave.total_budget.get() == U256::ZERO {
        return Err(Error::WaveNotFound.into());
    }
    Ok((
        wave.total_budget.get(),
        wave.total_points.get(),
        wave.points_finalized.get(),
        wave.end_timestamp.get(),
        wave.total_claimed.get(),
    ))
}

pub fn has_claimed(storage: &WaveEscrow, wave_id: U256, contributor: Address) -> bool {
    storage.has_claimed.getter(wave_id).getter(contributor)
}
