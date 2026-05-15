//! WaveEscrow — Milestone-based point locker for Drips reward distribution.
//!
//! An Arbitrum Stylus smart contract that holds USDC/USDT reward pools for
//! a specific Wave cycle and releases funds to contributors after the Drips
//! GitHub App verifies the earned points.

#![no_std]

extern crate alloc;
extern crate mini_alloc;

mod contract;
mod errors;
mod events;

pub use errors::Error;

use alloc::vec::Vec;
use alloy_primitives::{Address, U256};
use stylus_sdk::prelude::*;

sol_storage! {
    #[entrypoint]
    pub struct WaveEscrow {
        address owner;
        address wave_oracle;
        address reward_token;
        bool paused;
        uint256 wave_count;
        mapping(uint256 => Wave) waves;
        mapping(uint256 => mapping(address => bool)) has_claimed;
    }

    struct Wave {
        uint256 total_budget;
        uint256 total_points;
        bool points_finalized;
        uint256 end_timestamp;
        uint256 total_claimed;
    }
}

#[public]
impl WaveEscrow {
    // ── Wave lifecycle ──────────────────────────────────────────────────────

    fn initialize_wave(
        &mut self,
        wave_id: U256,
        budget: U256,
        end_timestamp: U256,
    ) -> Result<(), Vec<u8>> {
        contract::initialize_wave(self, wave_id, budget, end_timestamp)
    }

    fn finalize_wave_points(
        &mut self,
        wave_id: U256,
        total_points: U256,
    ) -> Result<(), Vec<u8>> {
        contract::finalize_wave_points(self, wave_id, total_points)
    }

    fn deposit_funds(&mut self, wave_id: U256, amount: U256) -> Result<(), Vec<u8>> {
        contract::deposit_funds(self, wave_id, amount)
    }

    fn claim_reward(
        &mut self,
        wave_id: U256,
        contributor_points: U256,
    ) -> Result<(), Vec<u8>> {
        contract::claim_reward(self, wave_id, contributor_points)
    }

    // ── Ownership & Oracle ──────────────────────────────────────────────────

    fn transfer_ownership(&mut self, new_owner: Address) -> Result<(), Vec<u8>> {
        contract::transfer_ownership(self, new_owner)
    }

    fn update_oracle(&mut self, new_oracle: Address) -> Result<(), Vec<u8>> {
        contract::update_oracle(self, new_oracle)
    }

    // ── Pausability ─────────────────────────────────────────────────────────

    fn pause(&mut self) -> Result<(), Vec<u8>> {
        contract::pause(self)
    }

    fn unpause(&mut self) -> Result<(), Vec<u8>> {
        contract::unpause(self)
    }

    // ── Emergency ───────────────────────────────────────────────────────────

    fn emergency_withdraw(
        &mut self,
        wave_id: U256,
        to: Address,
    ) -> Result<(), Vec<u8>> {
        contract::emergency_withdraw(self, wave_id, to)
    }

    // ── View functions ──────────────────────────────────────────────────────

    fn get_wave(
        &self,
        wave_id: U256,
    ) -> Result<(U256, U256, bool, U256, U256), Vec<u8>> {
        contract::get_wave(self, wave_id)
    }

    fn has_claimed(&self, wave_id: U256, contributor: Address) -> bool {
        contract::has_claimed(self, wave_id, contributor)
    }

    fn get_owner(&self) -> Address {
        self.owner.get()
    }

    fn get_oracle(&self) -> Address {
        self.wave_oracle.get()
    }

    fn is_paused(&self) -> bool {
        self.paused.get()
    }

    fn get_reward_token(&self) -> Address {
        self.reward_token.get()
    }
}
