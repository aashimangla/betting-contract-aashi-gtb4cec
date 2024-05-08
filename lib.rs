use soroban_sdk::{contractimpl, env, Address, Bytes, Env, IntoVal, Symbol};
struct Bet {
    better: Address,
    event_id: Symbol,
    outcome: Symbol, 
    wager: i128,
}
struct BettingContractData {
    bets: Vec<Bet>,
    oracle: Address, 
}
contractimpl! {
    pub fn place_bet(e: Env, event_id: Symbol, outcome: Symbol, wager: i128) {
        
        assert!(wager > 0, "Wager must be positive"); 

        let better = e.invoker();

        e.data().push_back(Bet {
            better,
            event_id,
            outcome,
            wager
        });
    }

    pub fn update_event_outcome(e: Env, event_id: Symbol, actual_outcome: Symbol) {
        assert!(e.invoker() == e.data().oracle, "Only the oracle can update outcomes");
    }
}
