# 📘 Contract Summary

This project is a Rust smart contract that manages and distributes a **scholarship fund**.

The idea is simple: an organization (like an NGO or a DAO) collects funds and then safely distributes them to eligible students. The contract focuses on doing this **securely, transparently, and without unnecessary complexity**.

What the contract allows an authority to do:

- Create and manage a scholarship pool
- Register eligible student addresses
- Distribute fixed or calculated scholarship amounts
- Prevent double claims and unauthorized access

The overall design is intentionally **simple, auditable, and easy to extend**, making it suitable for real-world education funding scenarios.

---

## 🛠️ How to Build & Test (Local)

This contract was **run and tested locally** and was not deployed to a public testnet.

### Build the contract

```bash
cargo build
```

### Local Test

```bash
cargo test
```

### Example

```bash
cargo run --example schema
```

### Output:

(images\output.png)

---

## 🧪 Local Execution

All important state transitions were tested locally using Rust unit tests that simulate real transaction flows.

Even though the contract is not deployed to a public testnet, the logic behaves exactly the same as it would in an on-chain environment. The tests cover normal usage as well as edge cases to ensure correctness.

---

## 🧠 Design Rationale

A few key design decisions guided this implementation:

- A **clear and explicit state machine** is used to avoid hidden or accidental state changes.
- All logic uses **Result-based error handling**, making failures explicit and predictable.
- **Strong access control** ensures only the authorized account can perform critical actions.
- **Simple data models** keep the contract easy to audit, understand, and extend.

The main goal was **clarity over cleverness**.

---

## 🔄 State & Flows

The contract follows a straightforward flow that mirrors how scholarship programs work in the real world.

### 1. Initialization

- The contract is initialized with an authority address.
- The initial scholarship pool balance is set.

### 2. Student Registration

- The authority registers eligible student addresses.
- Duplicate registrations are rejected.

### 3. Distribution

- Funds are distributed only to registered students.
- Each student can claim the scholarship **only once**.

### 4. Completion

- The scholarship pool balance updates after every transfer.
- The final contract state is deterministic and easy to verify.

---
