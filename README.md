![Equity Banner](/images/equity.png)

# Equity Token

Equity Token is an implementation of TokenInterface with features designed for buisnesses to launch equity, manage governance and setup configurable regulations for retail investors in mind.

## Project Structure

This repository uses the recommended structure for a Soroban project:
```text
.
├── contracts
│   └── EquityToken
│       ├── src
│       │   ├── lib.rs
│       │   └── test.rs
│       └── Cargo.toml
├── Cargo.toml
└── README.md
```

## Regulations 

The equity-token fundamentally implement these sets of regulations, but the scale is configurable initially by buisness owner and later based on community voting through governance.

```rust
  // Regulations are holders governed set of rules
  // for securing themselves from losses & sepculation.
  pub struct Regulations {
    // List of exchanges tokens can be traded on.
    pub exchange_whitelist: Vec<Address>,

    // KYC requirement for holders, credentials can be verified
    // on-chain using chosen protocol / logic.
    pub only_kyc_owners: bool,

    // Circuit breakers limit defining min/max price changes per day.
    pub circuit_limit: u32,

    // Maximum ownership possible per account.
    pub max_ownership: i128
  }
```

Read more: https://sebi.xyz 


### Registered Contracts

- Contract Latest: CB4DJBVLORRPY7GFJ64KHV7QWB5IDWNDHEV6KFFMPOVKKUDWFOZO2RQI
- Wasm Hash for Equity Token Contract:  699166484a3c6eadfc65184b416f2fff36635952c75e6ec6a779c78b3c5f990e
