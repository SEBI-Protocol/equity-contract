use soroban_sdk::{vec, Address, Env, String};
use soroban_token_sdk::TokenUtils;

use crate::storage_types::{DataKey, Regulations};

pub fn get_default_regulations(e: &Env, d_exchange: Address) -> Regulations {
    // Sets up the default configuration for the token.
    // Change this from hardcoded to proxy contract later.
    let regulations = Regulations {
        // List of exchanges tokens can be traded on.
        // SEBI Protocol's default exchange wallet here.
        exchange_whitelist: vec![&e, d_exchange],

        // KYC requirement for holders, credentials can be verified
        // on-chain using chosen protocol / logic.
        only_kyc_owners: false,

        // Circuit breakers limit defining min/max price changes per day.
        // Enforced in the exchange.
        circuit_limit: 15, // in percentage.

        // Maximum ownership possible per account.
        max_ownership: 100000,
    };

    regulations
}

pub fn read_is_kyc_required(e: &Env) -> bool {
    let key = DataKey::Regulations;
    let regulations: Regulations =
        e.storage()
            .persistent()
            .get(&key)
            .unwrap_or(get_default_regulations(
                &e,
                Address::from_string(&String::from_str(
                    &e,
                    "GC3HTLMTWGD7QVGBWRSKI2R2YEM2LIULUUKVLBWGMODQL5EDSL6N66FN",
                )),
            ));

    regulations.only_kyc_owners
}

pub fn read_max_ownership(e: &Env) -> i128 {
    let key = DataKey::Regulations;
    let regulations: Regulations =
        e.storage()
            .persistent()
            .get(&key)
            .unwrap_or(get_default_regulations(
                &e,
                Address::from_string(&String::from_str(
                    &e,
                    "GC3HTLMTWGD7QVGBWRSKI2R2YEM2LIULUUKVLBWGMODQL5EDSL6N66FN",
                )),
            ));

    regulations.max_ownership
}

pub fn read_circuit_limit(e: &Env) -> u32 {
    let key = DataKey::Regulations;
    let regulations: Regulations =
        e.storage()
            .persistent()
            .get(&key)
            .unwrap_or(get_default_regulations(
                &e,
                Address::from_string(&String::from_str(
                    &e,
                    "GC3HTLMTWGD7QVGBWRSKI2R2YEM2LIULUUKVLBWGMODQL5EDSL6N66FN",
                )),
            ));

    regulations.circuit_limit
}

pub fn check_whitelist(e: &Env, address: Address) -> bool {
    let key = DataKey::Regulations;
    let regulations: Regulations =
        e.storage()
            .persistent()
            .get(&key)
            .unwrap_or(get_default_regulations(
                &e,
                Address::from_string(&String::from_str(
                    &e,
                    "GC3HTLMTWGD7QVGBWRSKI2R2YEM2LIULUUKVLBWGMODQL5EDSL6N66FN",
                )),
            ));

    regulations.exchange_whitelist.contains(address)
}

pub fn write_default_regulations(e: &Env, d_exchange: Address) {
    // Sets up the default configuration for the token.
    // Change this from hardcoded to proxy contract later.
    let regulations = Regulations {
        // List of exchanges tokens can be traded on.
        // SEBI Protocol's default exchange wallet here.
        exchange_whitelist: vec![&e, d_exchange],

        // KYC requirement for holders, credentials can be verified
        // on-chain using chosen protocol / logic.
        only_kyc_owners: false,

        // Circuit breakers limit defining min/max price changes per day.
        // Enforced in the exchange.
        circuit_limit: 15, // in percentage.

        // Maximum ownership possible per account.
        max_ownership: 100000,
    };

    let key = DataKey::Regulations;
    e.storage().persistent().set(&key.clone(), &regulations);
}
