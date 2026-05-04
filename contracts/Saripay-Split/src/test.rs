#![cfg(test)]

use soroban_sdk::{Env, Address};
use crate::SariPaySplit;

#[test]
fn test_happy_path() {
    let env = Env::default();
    let contract = SariPaySplit;

    let store = Address::random(&env);
    let customer = Address::random(&env);

    // Add debt: ₱100
    contract.add_debt(env.clone(), store.clone(), customer.clone(), 100);

    // Repay ₱40
    contract.repay(env.clone(), customer.clone(), store.clone(), 40);

    let balance = contract.get_balance(env, customer, store);

    assert_eq!(balance, 60);
}

#[test]
fn test_edge_overpayment() {
    let env = Env::default();
    let contract = SariPaySplit;

    let store = Address::random(&env);
    let customer = Address::random(&env);

    contract.add_debt(env.clone(), store.clone(), customer.clone(), 50);

    // Try to overpay
    contract.repay(env.clone(), customer.clone(), store.clone(), 100);

    let balance = contract.get_balance(env, customer, store);

    // Should not go negative
    assert_eq!(balance, 0);
}

#[test]
fn test_state_verification() {
    let env = Env::default();
    let contract = SariPaySplit;

    let store = Address::random(&env);
    let customer = Address::random(&env);

    contract.add_debt(env.clone(), store.clone(), customer.clone(), 200);

    let balance = contract.get_balance(env, customer, store);

    assert_eq!(balance, 200);
}

#[test]
fn test_zero_balance_default() {
    let env = Env::default();
    let contract = SariPaySplit;

    let store = Address::random(&env);
    let customer = Address::random(&env);

    let balance = contract.get_balance(env, customer, store);

    assert_eq!(balance, 0);
}

#[test]
fn test_multiple_updates() {
    let env = Env::default();
    let contract = SariPaySplit;

    let store = Address::random(&env);
    let customer = Address::random(&env);

    contract.add_debt(env.clone(), store.clone(), customer.clone(), 100);
    contract.add_debt(env.clone(), store.clone(), customer.clone(), 50);

    contract.repay(env.clone(), customer.clone(), store.clone(), 30);

    let balance = contract.get_balance(env, customer, store);

    assert_eq!(balance, 120);
}