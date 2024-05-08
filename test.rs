use soroban_sdk::{contractimpl, testutils::Accounts, testutils::Ledger, Env, Symbol};

// Helper to simulate an account with XLM balance
fn create_account(e: &Env, balance: i128) -> Address {
    let key = e.ledger().contract_id().unwrap();
    e.ledger().set_balance(&key, balance);
    key
}

struct TestScenario {
    ledger: Ledger,
    env: Env,
    contract_id: Address,
}


fn setup_scenario() -> TestScenario {
    let e = Env::default();
    let ledger = Ledger::default();
    let contract_id = e.register_contract(None, BettingContractData{ ..Default::default() });
    TestScenario { ledger, env: e, contract_id }
}

#[test]
fn test_place_bet() {
    let scenario = setup_scenario();
    let user = create_account(&scenario.env, 1000); // Create a user with 1000 XLM

    
    scenario.env.set_invoker(&user);
    scenario.env.invoke_contract(
        &scenario.contract_id,
        "place_bet",
        &(&Symbol::from_str("EVENT1"), &Symbol::from_str("TEAM_A"), &100),
    );

}

#[test]
fn test_oracle_update() {
    let scenario = setup_scenario();
    let oracle = create_account(&scenario.env, 500);

    
    scenario.env.data().oracle = oracle.clone();
    scenario.env.set_invoker(&oracle);
    scenario.env.invoke_contract(
        &scenario.contract_id,
        "update_event_outcome",
        &(&Symbol::from_str("EVENT1"), &Symbol::from_str("TEAM_B")),
    );

}
