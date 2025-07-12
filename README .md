
# ⏳ Time-Decay Threshold Consensus

A Rust-based consensus voting engine where vote weight and decision thresholds evolve over time to optimize early participation, maintain fairness, and ensure network liveness.

---

## 🚀 Project Overview

**Time-Decay Threshold Consensus** introduces a novel voting model that incorporates:

- **Time-weighted voting**: Older votes carry more weight.
- **Dynamic thresholds**: Escalating requirements over time.
- **Real-time computation**: Responsive consensus progress.
- **Safety guarantees**: Cryptographic and mathematical validation.

---

## 🧩 Core Features

### Time-Weighted Voting System
- **Decay Models**: 
  - **Exponential**: Rapid early decay.
  - **Linear**: Gradual decline.
  - **Stepped**: Phase-based degradation.
- **Verifiable Timestamps**: Validator signatures ensure trusted vote times (via NTP).
- **Weight Floors**: Minimum 10% weight retained to preserve vote influence.
- **Real-Time Engine**: Continuously updates vote weights as votes stream in.

###  Dynamic Threshold Escalation Engine
- **Progressive Thresholds**:
  - Starts at base (e.g., 51%), increases over time.
  - Escalation types: linear, exponential, sigmoid, custom step.
- **Ceiling Limit**: Threshold capped at 90% to ensure decision liveness.
- **Emergency Override**: For critical proposals requiring instant quorum.
- **Formal Verification**: Safety properties mathematically validated.

###  Voting Window Management System
- **Window Types**: Short (5 min), Medium (30 min), Long (2 hours), or Custom.
- **Extensions**: Threshold-near-miss triggers deadline extension.
- **Cleanup**: Incomplete proposals are auto-expired and cleared.
- **Overlapping Proposals**: Concurrent voting allowed, based on urgency.
- **Grace Buffers**: Handle clock drift and network latency.

###  Weight Calculation Engine
- **Precision Arithmetic**: Prevents rounding errors.
- **Weight Caching**: Optimizes static vote processing.
- **Audit Trail**: Track vote weight history for transparency.
- **Batch Updates**: Efficient simultaneous vote processing.
- **Reputation Integration**: Trustworthy validators get weight bonuses.

###  Threshold Progression Framework
- **Profiles**: Conservative, Aggressive, Adaptive.
- **Scheduling**: Time-sensitive thresholds (e.g., stricter at night).
- **Multi-Dimensional Thresholds**:
  - Percentage consensus **and** minimum absolute vote count.
- **Proposal-Specific Requirements**:
  - Critical proposals demand stronger consensus.
- **Analytics**: Learn from past voting trends to fine-tune progression.

---

## 📁 File Structure

| File               | Description |
|--------------------|-------------|
| `main.rs`          | Entry point for the consensus simulation. |
| `vote.rs`          | Vote structure, timestamping, and decay models. |
| `threshold.rs`     | Threshold escalation logic and verification. |
| `weight_engine.rs` | Vote weight computation and caching. |
| `verify.rs`        | Timestamp validation using signatures and NTP. |
| `trust.rs`         | Validator reputation and bonus logic. |
| `window.rs`        | Proposal timing and voting window management. |
| `simulation.rs`    | Engine for simulating multiple proposals and time. |
| `history.rs`       | Tracks historical votes and outcomes. |

---

## 🔧 Build & Run

Ensure you have Rust installed. Then run:

```bash
cargo build
cargo run
````

## Testing
````
cargo test
```` 

---

## 📜 License

This project is licensed under the [MIT License](LICENSE)

---

## ✍️ Author

Built by Tanisha Mohapatra. Contributions and feedback welcome!


