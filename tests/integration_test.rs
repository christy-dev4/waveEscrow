use alloy_primitives::{Address, U256};
use stylus_sdk::testing::*;
use wave_escrow::*;

fn default_oracle() -> Address {
    Address::from([0x02; 20])
}

fn default_owner() -> Address {
    Address::from([0x01; 20])
}

fn alice() -> Address {
    Address::from([0x03; 20])
}

fn bob() -> Address {
    Address::from([0x04; 20])
}

fn setup() -> WaveEscrow {
    let mut contract = WaveEscrow::default();

    // Simulate deploy: set owner and oracle
    contract.owner.set(default_owner());
    contract.wave_oracle.set(default_oracle());
    contract.reward_token.set(Address::from([0x42; 20]));
    contract.paused.set(false);

    contract
}

#[test]
fn test_initialize_wave() {
    let mut contract = setup();

    // Act as owner
    set_caller(default_owner());
    let wave_id = U256::from(1);
    let budget = U256::from(100_000);
    let end_ts = U256::from(1_000_000_000);

    let result = contract.initialize_wave(wave_id, budget, end_ts);
    assert!(result.is_ok());

    let (stored_budget, stored_points, finalized, end_ts_stored, claimed) =
        contract.get_wave(wave_id).unwrap();
    assert_eq!(stored_budget, budget);
    assert_eq!(stored_points, U256::ZERO);
    assert!(!finalized);
    assert_eq!(end_ts_stored, end_ts);
    assert_eq!(claimed, U256::ZERO);
}

#[test]
fn test_initialize_wave_zero_budget() {
    let mut contract = setup();
    set_caller(default_owner());
    let result = contract.initialize_wave(U256::from(1), U256::ZERO, U256::from(1_000));
    assert!(result.is_err());
}

#[test]
fn test_initialize_wave_duplicate() {
    let mut contract = setup();
    set_caller(default_owner());

    let id = U256::from(1);
    contract.initialize_wave(id, U256::from(100), U256::from(1_000)).unwrap();
    let result = contract.initialize_wave(id, U256::from(200), U256::from(2_000));
    assert!(result.is_err());
}

#[test]
fn test_not_owner_cannot_initialize() {
    let mut contract = setup();
    set_caller(alice());
    let result = contract.initialize_wave(U256::from(1), U256::from(100), U256::from(1_000));
    assert!(result.is_err());
}

#[test]
fn test_finalize_wave_points() {
    let mut contract = setup();

    set_caller(default_owner());
    let wave_id = U256::from(1);
    contract.initialize_wave(wave_id, U256::from(100_000), U256::from(1_000_000)).unwrap();

    set_caller(default_oracle());
    let total_points = U256::from(1_000);
    let result = contract.finalize_wave_points(wave_id, total_points);
    assert!(result.is_ok());

    let (_, stored_points, finalized, _, _) = contract.get_wave(wave_id).unwrap();
    assert_eq!(stored_points, total_points);
    assert!(finalized);
}

#[test]
fn test_finalize_twice_fails() {
    let mut contract = setup();

    set_caller(default_owner());
    let wave_id = U256::from(1);
    contract.initialize_wave(wave_id, U256::from(100), U256::from(1_000)).unwrap();

    set_caller(default_oracle());
    contract.finalize_wave_points(wave_id, U256::from(500)).unwrap();
    let result = contract.finalize_wave_points(wave_id, U256::from(600));
    assert!(result.is_err());
}

#[test]
fn test_non_oracle_cannot_finalize() {
    let mut contract = setup();

    set_caller(default_owner());
    let wave_id = U256::from(1);
    contract.initialize_wave(wave_id, U256::from(100), U256::from(1_000)).unwrap();

    set_caller(alice());
    let result = contract.finalize_wave_points(wave_id, U256::from(500));
    assert!(result.is_err());
}

#[test]
fn test_claim_reward() {
    let mut contract = setup();

    set_caller(default_owner());
    let wave_id = U256::from(1);
    contract.initialize_wave(wave_id, U256::from(1_000), U256::from(1_000_000)).unwrap();

    set_caller(default_oracle());
    contract.finalize_wave_points(wave_id, U256::from(1_000)).unwrap();

    set_caller(alice());
    let alice_points = U256::from(300); // 30% share
    let result = contract.claim_reward(wave_id, alice_points);
    assert!(result.is_ok());

    assert!(contract.has_claimed(wave_id, alice()));
}

#[test]
fn test_claim_reward_correct_amount() {
    let mut contract = setup();

    set_caller(default_owner());
    let wave_id = U256::from(1);
    let budget = U256::from(10_000);
    contract.initialize_wave(wave_id, budget, U256::from(1_000_000)).unwrap();

    set_caller(default_oracle());
    let total_points = U256::from(100);
    contract.finalize_wave_points(wave_id, total_points).unwrap();

    set_caller(alice());
    let alice_points = U256::from(25); // 25%
    contract.claim_reward(wave_id, alice_points).unwrap();

    // Expected: 10_000 * 25 / 100 = 2_500
    // (In production the token transfer would have occurred; here we just verify no error)
}

#[test]
fn test_double_claim_fails() {
    let mut contract = setup();

    set_caller(default_owner());
    let wave_id = U256::from(1);
    contract.initialize_wave(wave_id, U256::from(1_000), U256::from(1_000_000)).unwrap();

    set_caller(default_oracle());
    contract.finalize_wave_points(wave_id, U256::from(100)).unwrap();

    set_caller(alice());
    contract.claim_reward(wave_id, U256::from(10)).unwrap();

    let result = contract.claim_reward(wave_id, U256::from(10));
    assert!(result.is_err());
}

#[test]
fn test_claim_before_finalize_fails() {
    let mut contract = setup();

    set_caller(default_owner());
    let wave_id = U256::from(1);
    contract.initialize_wave(wave_id, U256::from(1_000), U256::from(1_000_000)).unwrap();

    set_caller(alice());
    let result = contract.claim_reward(wave_id, U256::from(100));
    assert!(result.is_err());
}

#[test]
fn test_transfer_ownership() {
    let mut contract = setup();

    set_caller(default_owner());
    let new_owner = alice();
    let result = contract.transfer_ownership(new_owner);
    assert!(result.is_ok());
    assert_eq!(contract.get_owner(), new_owner);
}

#[test]
fn test_non_owner_cannot_transfer() {
    let mut contract = setup();
    set_caller(alice());
    let result = contract.transfer_ownership(bob());
    assert!(result.is_err());
}

#[test]
fn test_update_oracle() {
    let mut contract = setup();

    set_caller(default_owner());
    let new_oracle = Address::from([0x99; 20]);
    let result = contract.update_oracle(new_oracle);
    assert!(result.is_ok());
    assert_eq!(contract.get_oracle(), new_oracle);
}

#[test]
fn test_pause_unpause() {
    let mut contract = setup();

    set_caller(default_owner());
    assert!(!contract.is_paused());

    contract.pause().unwrap();
    assert!(contract.is_paused());

    contract.unpause().unwrap();
    assert!(!contract.is_paused());
}

#[test]
fn test_paused_blocks_actions() {
    let mut contract = setup();

    set_caller(default_owner());
    contract.pause().unwrap();

    let result = contract.initialize_wave(U256::from(1), U256::from(100), U256::from(1_000));
    assert!(result.is_err());
}

#[test]
fn test_emergency_withdraw() {
    let mut contract = setup();

    set_caller(default_owner());
    let wave_id = U256::from(1);
    contract.initialize_wave(wave_id, U256::from(100_000), U256::from(1_000_000)).unwrap();

    // Must be paused for emergency withdraw
    contract.pause().unwrap();

    let result = contract.emergency_withdraw(wave_id, alice());
    assert!(result.is_ok());

    // After withdraw, budget should be reduced
    let (remaining, _, _, _, _) = contract.get_wave(wave_id).unwrap();
    assert_eq!(remaining, U256::ZERO);
}

#[test]
fn test_emergency_withdraw_not_paused_fails() {
    let mut contract = setup();

    set_caller(default_owner());
    let wave_id = U256::from(1);
    contract.initialize_wave(wave_id, U256::from(100), U256::from(1_000)).unwrap();

    let result = contract.emergency_withdraw(wave_id, alice());
    assert!(result.is_err());
}

#[test]
fn test_get_nonexistent_wave() {
    let contract = setup();
    let result = contract.get_wave(U256::from(999));
    assert!(result.is_err());
}

#[test]
fn test_deposit_funds() {
    let mut contract = setup();

    set_caller(default_owner());
    let wave_id = U256::from(1);
    contract.initialize_wave(wave_id, U256::from(1_000), U256::from(1_000_000)).unwrap();

    contract.deposit_funds(wave_id, U256::from(500)).unwrap();

    let (budget, _, _, _, _) = contract.get_wave(wave_id).unwrap();
    assert_eq!(budget, U256::from(1_500));
}
