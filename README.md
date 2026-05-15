# WaveEscrow

**Milestone-based point locker for Drips reward distribution on Arbitrum Stylus.**

Holds USDC/USDT reward pools for a specific Wave cycle and releases funds to contributors after the Drips GitHub App verifies the points earned during the Sprint. Automates the "Reward" phase of the Drips lifecycle — instead of maintainers manually sending payouts, this contract holds the funds and distributes them based on point-share math.

---

## How It Works

1. **Initialize Wave** — Owner sets a `wave_id`, `total_budget`, and `end_timestamp`.
2. **Finalize Points** — The Drips oracle submits a signed payload with the total earned points for the wave.
3. **Claim Reward** — Each contributor calls `claim_reward` with their earned `contributor_points`. The contract calculates their pro-rata share:

```
Payout = Total Budget × (Contributor Points / Total Wave Points)
```

---

## Contract Architecture

```
WaveEscrow
├── initialize_wave(wave_id, budget, end_timestamp)     [owner]
├── finalize_wave_points(wave_id, total_points)          [oracle]
├── deposit_funds(wave_id, amount)                       [anyone]
├── claim_reward(wave_id, contributor_points)            [contributor]
├── transfer_ownership(new_owner)                        [owner]
├── update_oracle(new_oracle)                            [owner]
├── pause() / unpause()                                  [owner]
├── emergency_withdraw(wave_id, to)                      [owner, paused]
└── View functions: get_wave(), has_claimed(), get_owner(), get_oracle(), is_paused()
```

### Storage Layout

```
WaveEscrow (entrypoint)
├── owner:              address
├── wave_oracle:        address
├── reward_token:       address
├── paused:             bool
├── wave_count:         uint256
├── waves:              mapping(uint256 => Wave)
│   └── Wave
│       ├── total_budget:     uint256
│       ├── total_points:     uint256
│       ├── points_finalized: bool
│       ├── end_timestamp:    uint256
│       └── total_claimed:    uint256
└── has_claimed:        mapping(uint256 => mapping(address => bool))
```

### Events

| Event | Description |
|-------|-------------|
| `WaveInitialized` | A new wave reward pool is created |
| `PointsFinalized` | Oracle submits total points for a wave |
| `RewardClaimed` | Contributor claims their payout |
| `FundsDeposited` | Additional funds added to a wave budget |
| `OwnershipTransferred` | Contract ownership changes |
| `OracleUpdated` | Oracle address updated |
| `EmergencyWithdraw` | Owner recovers remaining funds (paused only) |
| `ContractPaused` / `ContractUnpaused` | Emergency pause toggled |

---

## Prerequisites

- [Rust](https://rustup.rs/) nightly toolchain
- Wasm target: `rustup target add wasm32-unknown-unknown`
- `cargo-stylus`: `cargo install cargo-stylus`

## Build

```bash
cargo stylus build
```

Compiles to a WASM binary ready for Arbitrum Stylus deployment.

## Test

```bash
cargo test
```

Runs the test suite on the host target (no WASM needed). Covers:

- Wave initialization (success, zero budget, duplicate, unauthorized)
- Point finalization (success, double-finalize, unauthorized)
- Reward claiming (success, pro-rata math, double-claim, pre-finalization)
- Access control (ownership transfer, oracle update)
- Pausability (pause, unpause, blocked actions)
- Emergency withdrawal
- Deposit funds
- Edge cases (nonexistent wave, zero values)

## Deploy

### 1. Set environment variables

```bash
export RPC_URL=https://sepolia-rollup.arbitrum.io/rpc
export PRIVATE_KEY=0x...
```

### 2. Deploy

```bash
bash scripts/deploy.sh
```

Or manually:

```bash
cargo stylus deploy \
    --endpoint "$RPC_URL" \
    --private-key "$PRIVATE_KEY"
```

### 3. Initialize

Once deployed, call `initialize_wave` with your wave parameters. The oracle address must be set during contract initialization (via `update_oracle`).

---

## Security

- **Access control**: Owner-only and oracle-only guards on privileged functions.
- **Checks-effects-interactions**: Claim state updates before any external calls.
- **Double-claim prevention**: Each contributor can claim each wave once.
- **Pausability**: Emergency pause halts all state mutations.
- **Emergency withdraw**: Owner can recover funds only when paused.
- **Input validation**: All public functions validate inputs (zero values, timestamps, duplicate waves).

---

## Project Structure

```
waveEscrow/
├── .cargo/config.toml
├── .github/workflows/ci.yml
├── Cargo.toml
├── build.rs
├── rust-toolchain.toml
├── scripts/deploy.sh
├── src/
│   ├── lib.rs          # sol_storage! + #[public] ABI
│   ├── contract.rs     # Business logic
│   ├── errors.rs       # Error types
│   └── events.rs       # Event definitions
└── tests/
    └── integration_test.rs
```

---

## License

MIT
