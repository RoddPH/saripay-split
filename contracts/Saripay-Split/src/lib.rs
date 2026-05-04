#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype,
    Env, Address, Symbol, Map
};

#[contract]
pub struct SariPaySplit;

// Unique key for each customer-store pair
#[contracttype]
#[derive(Clone)]
pub struct DebtKey {
    pub customer: Address,
    pub store: Address,
}

#[contractimpl]
impl SariPaySplit {

    // Store adds debt (utang)
    pub fn add_debt(env: Env, store: Address, customer: Address, amount: i128) {
        // Only store can add debt
        store.require_auth();

        let mut debts: Map<DebtKey, i128> = env
            .storage()
            .instance()
            .get(&Symbol::short("debts"))
            .unwrap_or(Map::new(&env));

        let key = DebtKey {
            customer: customer.clone(),
            store: store.clone(),
        };

        let current = debts.get(key.clone()).unwrap_or(0);
        debts.set(key, current + amount);

        env.storage().instance().set(&Symbol::short("debts"), &debts);
    }

    // Customer repays debt
    pub fn repay(env: Env, customer: Address, store: Address, amount: i128) {
        // Only customer can repay
        customer.require_auth();

        let mut debts: Map<DebtKey, i128> = env
            .storage()
            .instance()
            .get(&Symbol::short("debts"))
            .unwrap_or(Map::new(&env));

        let key = DebtKey {
            customer: customer.clone(),
            store: store.clone(),
        };

        let current = debts.get(key.clone()).unwrap_or(0);

        // Prevent overpayment going negative (simple guard)
        let new_balance = if amount > current { 0 } else { current - amount };

        debts.set(key, new_balance);

        env.storage().instance().set(&Symbol::short("debts"), &debts);
    }

    // View current debt
    pub fn get_balance(env: Env, customer: Address, store: Address) -> i128 {
        let debts: Map<DebtKey, i128> = env
            .storage()
            .instance()
            .get(&Symbol::short("debts"))
            .unwrap_or(Map::new(&env));

        let key = DebtKey { customer, store };

        debts.get(key).unwrap_or(0)
    }
}