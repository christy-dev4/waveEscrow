use alloy_primitives::{Address, U256};
use alloy_sol_types::{sol, SolEvent};
use stylus_sdk::evm;

sol! {
    event WaveInitialized(uint256 indexed wave_id, uint256 budget, uint256 end_timestamp, address indexed owner);
    event PointsFinalized(uint256 indexed wave_id, uint256 total_points, address indexed oracle);
    event RewardClaimed(uint256 indexed wave_id, address indexed contributor, uint256 reward);
    event FundsDeposited(uint256 indexed wave_id, address indexed depositor, uint256 amount);
    event OwnershipTransferred(address indexed previous_owner, address indexed new_owner);
    event OracleUpdated(address indexed previous_oracle, address indexed new_oracle);
    event EmergencyWithdraw(uint256 indexed wave_id, address indexed to, uint256 amount);
    event ContractPaused(address account);
    event ContractUnpaused(address account);
}

pub fn wave_initialized(wave_id: U256, budget: U256, end_timestamp: U256, owner: Address) {
    evm::log(WaveInitialized { wave_id, budget, end_timestamp, owner }.encode());
}

pub fn points_finalized(wave_id: U256, total_points: U256, oracle: Address) {
    evm::log(PointsFinalized { wave_id, total_points, oracle }.encode());
}

pub fn reward_claimed(wave_id: U256, contributor: Address, reward: U256) {
    evm::log(RewardClaimed { wave_id, contributor, reward }.encode());
}

pub fn funds_deposited(wave_id: U256, depositor: Address, amount: U256) {
    evm::log(FundsDeposited { wave_id, depositor, amount }.encode());
}

pub fn ownership_transferred(previous_owner: Address, new_owner: Address) {
    evm::log(OwnershipTransferred { previous_owner, new_owner }.encode());
}

pub fn oracle_updated(previous_oracle: Address, new_oracle: Address) {
    evm::log(OracleUpdated { previous_oracle, new_oracle }.encode());
}

pub fn emergency_withdraw(wave_id: U256, to: Address, amount: U256) {
    evm::log(EmergencyWithdraw { wave_id, to, amount }.encode());
}

pub fn paused(account: Address) {
    evm::log(ContractPaused { account }.encode());
}

pub fn unpaused(account: Address) {
    evm::log(ContractUnpaused { account }.encode());
}
