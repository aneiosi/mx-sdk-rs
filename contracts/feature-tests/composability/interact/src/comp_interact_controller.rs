use crate::{call_tree::CallState, comp_interact_config::Config, comp_interact_state::State};

use multiversx_sc_snippets::{
    multiversx_sc::types::Address, multiversx_sc_scenario::test_wallets::judy, Interactor,
};

const INTERACTOR_SCENARIO_TRACE_PATH: &str = "comp_interact_trace.scen.json";

pub struct ComposabilityInteract {
    pub interactor: Interactor,
    pub wallet_address: Address,
    pub state: State,
}

impl ComposabilityInteract {
    pub async fn init() -> Self {
        let config = Config::load_config();
        let mut interactor = Interactor::new(config.gateway())
            .await
            .with_tracer(INTERACTOR_SCENARIO_TRACE_PATH)
            .await;
        let wallet_address = interactor.register_wallet(judy());

        ComposabilityInteract {
            interactor,
            wallet_address,
            state: State::load_state(),
        }
    }

    pub async fn full_scenario(&mut self, endpoint_name: &str, _endpoint_args: &Option<Vec<String>>) {
        let config = Config::load_config();
        let payment_token = config.token_id();
        let call_type = config.call_type();
        let payment_amount = config.token_amount();
        let payment_nonce = config.token_nonce();

        let call_state = CallState::simple_example_2();
        call_state.print();

        self.deploy_call_tree_contracts(&call_state).await;

        self.add_calls_to_all_fwds(&call_state, call_type, endpoint_name)
            .await;

        self.call_root(&call_state, payment_token, payment_nonce, payment_amount)
            .await;
    }
}
