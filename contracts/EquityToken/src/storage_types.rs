use soroban_sdk::{contracttype, Address, Vec, U256};

pub(crate) const DAY_IN_LEDGERS: u32 = 17280;
pub(crate) const INSTANCE_BUMP_AMOUNT: u32 = 7 * DAY_IN_LEDGERS;
pub(crate) const INSTANCE_LIFETIME_THRESHOLD: u32 = INSTANCE_BUMP_AMOUNT - DAY_IN_LEDGERS;

pub(crate) const BALANCE_BUMP_AMOUNT: u32 = 30 * DAY_IN_LEDGERS;
pub(crate) const BALANCE_LIFETIME_THRESHOLD: u32 = BALANCE_BUMP_AMOUNT - DAY_IN_LEDGERS;

#[derive(Clone)]
#[contracttype]
pub struct AllowanceDataKey {
    pub from: Address,
    pub spender: Address,
}

#[contracttype]
pub struct AllowanceValue {
    pub amount: i128,
    pub expiration_ledger: u32,
}

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Allowance(AllowanceDataKey),
    Balance(Address),
    State(Address),
    Admin,
    Regulations
}

#[derive(Clone)]
#[contracttype]
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